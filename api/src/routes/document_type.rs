use diesel::prelude::*;
use rocket::{
    response::status::Created, 
	serde::json::Json, http::Status
};
use validator::Validate;

use crate::models::document_type::{DocumentType, NewDocumentType, UpdateDocumentType};
use crate::errors::ApiError;
use crate::schema::document_type;
use crate::db::Db;

#[get("/document-types")]
pub async fn fetch(
	db: Db
) -> Result<Json<Vec<DocumentType>>, (Status, Json<ApiError>)> {	
	db
		.run(|c| document_type::table
			.order_by(document_type::id.asc())
			.load(c)
		)
		.await
		.map(Json)
		.map_err(|e| ApiError::internal_server_error(&e.to_string())
	)
}

#[get("/document-types/<id>")]
pub async fn get(
	db: Db,
	id: i64
) -> Result<Json<DocumentType>, (Status, Json<ApiError>)> {
	db
		.run(move |c| document_type::table
			.find(id)
			.first(c)
		)
		.await
		.map(Json)
		.map_err(|e| ApiError::internal_server_error(&e.to_string())
	)
}

#[post("/document-types", format = "json", data = "<document_type>")]
pub async fn create(
	db: Db,
	document_type: Json<NewDocumentType>
) -> Result<Created<Json<DocumentType>>, (Status, Json<ApiError>)> {
	if let Err(e) = document_type.validate() {
		return Err(ApiError::validation_error(e))
	};

	db
		.run(move |c| {
			diesel::insert_into(document_type::table)
				.values(&document_type.into_inner())
				.get_result(c)
		})
		.await
		.map(|response| Created::new("/document-types").body(Json(response)))
		.map_err(|e| ApiError::internal_server_error(&e.to_string())
	)
}

#[rocket::patch("/document-types/<id>", format = "json", data = "<document_type>")]
pub async fn update(
	db: Db,
	id: i64,
	document_type: Json<UpdateDocumentType>
) -> Result<Json<DocumentType>, (Status, Json<ApiError>)> {
	if let Err(e) = document_type.validate() {
		return Err(ApiError::validation_error(e))
	};

	db
		.run(move|c| {
			diesel::update(document_type::table.find(id))
				.set(&document_type.into_inner())
				.get_result(c)
		})
		.await
		.map(Json)
		.map_err(|e| ApiError::internal_server_error(&e.to_string())
	)
}

#[delete("/document-types/<id>")]
pub async fn delete(
	db: Db,
	id: i64
) -> Result<Json<usize>, (Status, Json<ApiError>)> {
	db
		.run(move |c| {
			diesel::delete(
				document_type::table.find(id)
			)
			.execute(c)
		})
		.await
		.map(Json)
		.map_err(|e| ApiError::internal_server_error(&e.to_string())
	)
}