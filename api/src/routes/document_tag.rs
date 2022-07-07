use diesel::prelude::*;
use rocket::{
    response::status::Created, 
	serde::json::Json, http::Status
};
use validator::Validate;

use crate::models::document_tag::{DocumentTag, NewDocumentTag};
use crate::models::tag::Tag;
use crate::errors::ApiError;
use crate::schema::document_tag;
use crate::schema::tag;
use crate::db::Db;

#[get("/documents/<id>/tags")]
pub async fn fetch(
	db: Db,
	id: i64
) -> Result<Json<Vec<Tag>>, (Status, Json<ApiError>)> {
	db
	.run(move |c| document_tag::table
		.filter(document_tag::document_id.eq(id))
		.inner_join(tag::table)
		.select(tag::all_columns)
		.load(c)
	)
	.await
	.map(Json)
	.map_err(|e| ApiError::internal_server_error(&e.to_string())
)
}

#[post("/documents/<id>/tags/attach", format = "json", data = "<document_tag>")]
pub async fn attach(
	db: Db,
	id: i64,
	document_tag: Json<NewDocumentTag>
) -> Result<Created<Json<DocumentTag>>, (Status, Json<ApiError>)> {
	if let Err(e) = document_tag.validate() {
		return Err(ApiError::validation_error(e))
	};

	db
		.run(move |c| {
			diesel::insert_into(document_tag::table)
				.values(NewDocumentTag {
					document_id: Some(id),
					tag_id: document_tag.tag_id
				})
				.get_result(c)
		})
		.await
		.map(|response| Created::new("/document_tag").body(Json(response)))
		.map_err(|e| ApiError::internal_server_error(&e.to_string())
	)
}

#[post("/documents/<id>/tags/attach-multiple", format = "json", data = "<document_tags>")]
pub async fn attach_multiple(
	db: Db,
	id: i64,
	document_tags: Json<Vec<NewDocumentTag>>
) -> Result<Created<Json<Vec<DocumentTag>>>, (Status, Json<ApiError>)> {
	for document_tag in document_tags.clone().into_inner().iter() {
		if let Err(e) = document_tag.validate() {
			return Err(ApiError::validation_error(e))
		};
	}

	let document_tags: Vec<NewDocumentTag> = document_tags.into_inner().into_iter().map(|mut document_tag| {

		document_tag.document_id = Some(id);
		document_tag
	})
	.collect();


	db
		.run(move |c| {
			diesel::insert_into(document_tag::table)
				.values(document_tags)
				.get_results::<DocumentTag>(c)
		})
		.await
		.map(|response| Created::new("/document_tag").body(Json(response)))
		.map_err(|e| ApiError::internal_server_error(&e.to_string())
	)
}

#[delete("/documents/<id>/tags/remove/<tag_id>")]
pub async fn remove(
	db: Db,
	id: i64,
	tag_id: i64
) -> Result<Json<usize>, (Status, Json<ApiError>)> {
	db
		.run(move |c| {
			diesel::delete(
				document_tag::table
					.filter(document_tag::document_id.eq(id))
					.filter(document_tag::tag_id.eq(tag_id))
			)
			.execute(c)
		})
		.await
		.map(Json)
		.map_err(|e| ApiError::internal_server_error(&e.to_string())
	)
}