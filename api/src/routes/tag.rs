use diesel::prelude::*;
use rocket::{
    response::status::Created, 
	serde::json::Json, http::Status
};
use validator::Validate;

use crate::models::tag::{Tag, NewTag, UpdateTag};
use crate::errors::ApiError;
use crate::schema::tag;
use crate::db::Db;

#[get("/tags")]
pub async fn fetch(
	db: Db
) -> Result<Json<Vec<Tag>>, (Status, Json<ApiError>)> {	
	db
		.run(|c| tag::table
			.order_by(tag::id.asc())
			.load(c)
		)
		.await
		.map(Json)
		.map_err(|e| ApiError::internal_server_error(&e.to_string())
	)
}

#[get("/tags/<id>")]
pub async fn get(
	db: Db,
	id: i64
) -> Result<Json<Tag>, (Status, Json<ApiError>)> {
	db
		.run(move |c| tag::table
			.find(id)
			.first(c)
		)
		.await
		.map(Json)
		.map_err(|e| ApiError::internal_server_error(&e.to_string())
	)
}

#[post("/tags", format = "json", data = "<tag>")]
pub async fn create(
	db: Db,
	tag: Json<NewTag>
) -> Result<Created<Json<Tag>>, (Status, Json<ApiError>)> {
	if let Err(e) = tag.validate() {
		return Err(ApiError::validation_error(e))
	};

	db
		.run(move |c| {
			diesel::insert_into(tag::table)
				.values(&tag.into_inner())
				.get_result(c)
		})
		.await
		.map(|response| Created::new("/tags").body(Json(response)))
		.map_err(|e| ApiError::internal_server_error(&e.to_string())
	)
}

#[patch("/tags/<id>", format = "json", data = "<tag>")]
pub async fn update(
	db: Db,
	id: i64,
	tag: Json<UpdateTag>
) -> Result<Json<Tag>, (Status, Json<ApiError>)> {
	if let Err(e) = tag.validate() {
		return Err(ApiError::validation_error(e))
	};

	db
		.run(move|c| {
			diesel::update(tag::table.find(id))
				.set(&tag.into_inner())
				.get_result(c)
		})
		.await
		.map(Json)
		.map_err(|e| ApiError::internal_server_error(&e.to_string())
	)
}

#[delete("/tags/<id>")]
pub async fn delete(
	db: Db,
	id: i64
) -> Result<Json<usize>, (Status, Json<ApiError>)> {
	db
		.run(move |c| {
			diesel::delete(
				tag::table.find(id)
			)
			.execute(c)
		})
		.await
		.map(Json)
		.map_err(|e| ApiError::internal_server_error(&e.to_string())
	)
}