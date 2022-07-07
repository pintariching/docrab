use diesel::{QueryDsl, RunQueryDsl};
use diesel::prelude::*;

use rocket::http::Status;
use rocket::serde::json::Json;
use serde::{Serialize, Deserialize};
use validator::Validate;

use crate::db::Db;
use crate::errors::ApiError;
use crate::schema::{
	document,
	document_type, tag
};

use crate::models::{
	tag::Tag,
	document_type::DocumentType
};

use super::document_tag::DocumentTag;

const MAX_ID: i64 = i64::MAX;

#[derive(Queryable, Identifiable, Serialize, Debug)]
#[diesel(table_name = document)]
pub struct FullDocument {
	pub id: i64,
	pub label: String,
	pub document_type_id: i64,
	pub document_type: DocumentType,
	pub tags: Option<Vec<Tag>>,
	//pub files: Option<Vec<DocumentFile>>,
}

impl FullDocument {
	pub fn new(document: Document, tags: Vec<Tag>) -> FullDocument {
		FullDocument {
			id: document.id,
			label: document.label,
			document_type_id: document.document_type_id,
			document_type: document.document_type,
			tags: Some(tags)
		}
	}
}

#[derive(Queryable, Identifiable, Serialize, Clone, Debug)]
#[diesel(table_name = document)]
pub struct Document {
	pub id: i64,
	pub label: String,
	pub document_type_id: i64,
	pub document_type: DocumentType,
}

#[derive(Insertable, Selectable, Queryable, Validate, Deserialize)]
#[diesel(table_name = document)]
pub struct NewDocument {
	#[validate(
		required(),
		length(max = 100)
	)]
	pub label: Option<String>,
	
	#[validate(range(min = 1, max = "MAX_ID"))]
	pub document_type_id: i64,
}

#[derive(AsChangeset, Deserialize, Validate)]
#[diesel(table_name = document)]
pub struct UpdateDocument {
	#[validate(length(max = 100))]
	pub label: Option<String>,
	#[validate(range(min = 1, max = "MAX_ID"))]
	pub document_type_id: Option<i64>,
}

pub async fn get_document_from_id(id: i64, db: &Db) -> Result<Document, (Status, Json<ApiError>)> {
	db
		.run(move |c| document::table
			.find(id)
			.inner_join(document_type::table)
			.select((
				document::id,
				document::label,
				document::document_type_id,
				document_type::all_columns
			))
			.first::<Document>(c)
		)
		.await
		.map_err(|e| ApiError::internal_server_error(&e.to_string()))
}

pub async fn get_tags_for_document(document: Document, db: &Db) -> Result<Vec<Tag>, (Status, Json<ApiError>)> {
	db
		.run(move |c| DocumentTag::belonging_to(&document)
		.inner_join(tag::table)
		.select(tag::all_columns)
		.load::<Tag>(c)	
		)
		.await
		.map_err(|e| ApiError::internal_server_error(&e.to_string()))
}

pub async fn get_full_document_from_id(id: i64, db: &Db) -> Result<FullDocument, (Status, Json<ApiError>)> {
	let document = match get_document_from_id(id, &db).await {
		Ok(d) => d,
		Err(e) => return Err(e)
	};

	let tags = match get_tags_for_document(document.clone(), &db).await {
		Ok(t) => t,
		Err(e) => return Err(e)
	};

	Ok(FullDocument::new(document, tags))
}