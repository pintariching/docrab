use diesel::prelude::*;
use rocket::{
    response::status::Created,
	serde::json::Json, http::Status
};
use validator::Validate;

use crate::models::document::{Document, NewDocument, UpdateDocument, FullDocument, get_full_document_from_id};
use crate::errors:: ApiError;
use crate::models::document_tag::DocumentTag;
use crate::models::tag::Tag;
use crate::schema::{document, document_type, tag};
use crate::db::Db;

#[get("/documents")]
pub async fn fetch(
	db: Db
) -> Result<Json<Vec<FullDocument>>, (Status, Json<ApiError>)> {	
	let documents: Vec<Document> = match db.run(|c| document::table
		.order_by(document::id.asc())
		.inner_join(document_type::table)
		.select((
			document::id,
			document::label,
			document::document_type_id,
			document_type::all_columns
		))
		.load::<Document>(c)
		)
		.await
		.map_err(|e| ApiError::internal_server_error(&e.to_string())) {
			Ok(d) => d,
			Err(e) => return Err(e)
		};
	
	let document_clone_for_document_tags = documents.clone();
	let document_tags = match db.run(move |c| DocumentTag::belonging_to(&document_clone_for_document_tags)
			.inner_join(tag::table)
			.load::<(DocumentTag, Tag)>(c)
		)
		.await
		.map_err(|e| ApiError::internal_server_error(&e.to_string())) {
			Ok(dt) => dt,
			Err(e) => return Err(e)
	};

	let grouped_tags: Vec<Vec<Tag>> = document_tags.grouped_by(&documents)
		.into_iter()
		.map(|document_tag_tag|
			document_tag_tag.into_iter()
			.map(|(_, tag)| {
				tag
			})
			.collect()
		)
		.collect();

	let documents = documents.into_iter()
		.zip(grouped_tags)
		.map(|(doc, t)| {
			FullDocument::new(doc, t)
		})
		.collect::<Vec<_>>();
	
	Ok(Json(documents))
}

#[get("/documents/<id>")]
pub async fn get(
	db: Db,
	id: i64
) -> Result<Json<FullDocument>, (Status, Json<ApiError>)> {
	let document = match get_full_document_from_id(id, &db).await {
		Ok(d) => d,
		Err(e) => return Err(e)
	};

	Ok(Json(document))
}

#[post("/documents", format = "json", data = "<document>")]
pub async fn create(
	db: Db,
	document: Json<NewDocument>
) -> Result<Created<Json<FullDocument>>, (Status, Json<ApiError>)> {
	if let Err(e) = document.validate() {
		return Err(ApiError::validation_error(e))
	};

	let id = match db
		.run(move |c| {
			diesel::insert_into(document::table)
				.values(&document.into_inner())
				.returning(document::id)
				.get_result::<i64>(c)
		})
		.await
		.map_err(|e| ApiError::internal_server_error(&e.to_string())) {
			Ok(d) => d,
			Err(e) => return Err(e)
	};

	let document = match get_full_document_from_id(id, &db).await {
		Ok(d) => d,
		Err(e) => return Err(e)
	};

	Ok(Created::new("/documents")
		.body(Json(document)))
}

#[patch("/documents/<id>", format = "json", data = "<document>")]
pub async fn update(
	db: Db,
	id: i64,
	document: Json<UpdateDocument>
) -> Result<Json<FullDocument>, (Status, Json<ApiError>)> {
	if let Err(e) = document.validate() {
		return Err(ApiError::validation_error(e))
	};

	let id = match db
		.run(move|c| {
			diesel::update(document::table.find(id))
				.set(&document.into_inner())
				.returning(document::id)
				.get_result::<i64>(c)
		})
		.await
		.map_err(|e| ApiError::internal_server_error(&e.to_string())) {
			Ok(d) => d,
			Err(e) => return Err(e)
	};
	
	let document = match get_full_document_from_id(id, &db).await {
		Ok(d) => d,
		Err(e) => return Err(e)
	};

	Ok(Json(document))
}

#[delete("/documents/<id>")]
pub async fn delete(
	db: Db,
	id: i64
) -> Result<Json<usize>, (Status, Json<ApiError>)> {
	db
		.run(move |c| {
			diesel::delete(
				document::table.find(id)
			)
			.execute(c)
		})
		.await
		.map(Json)
		.map_err(|e| ApiError::internal_server_error(&e.to_string())
	)
}