use diesel::prelude::*;
use rocket::fs::TempFile;
use rocket::State;
use rocket::http::Status;
use rocket::response::status::{NoContent, Created};
use rocket::serde::json::Json;
use rocket::form::Form;

use crate::broker::Mq;
use crate::db::Db;
use crate::files::{FileDirectory, create_file};
use crate::models::document_file::DocumentFile;
use crate::errors::{ApiError};
use crate::schema::document_file;

use docrab_lib::{Task, RoutingKeys, JobPayload};

#[get("/documents/<document_id>/files")]
pub async fn fetch(
	db: Db,
	document_id: i64
) -> Result<Json<Vec<DocumentFile>>, (Status, Json<ApiError>)> {
	db
		.run(move |c| document_file::table
			.filter(document_file::document_id.eq(document_id))
			.load(c)
		)
		.await
		.map(Json)
		.map_err(|e| ApiError::internal_server_error(&e.to_string()))
}

#[get("/documents/<id>/files/<file_id>")]
pub async fn get(
	db: Db,
	id: i64,
	file_id: i64
) -> Result<Json<DocumentFile>, (Status, Json<ApiError>)> {
	db
		.run(move |c| document_file::table
			.filter(document_file::document_id.eq(id)
			.and(document_file::id.eq(file_id)))
			.first(c)
		)
		.await
		.map(Json)
		.map_err(|e| ApiError::internal_server_error(&e.to_string()))
}

#[post("/documents/<id>/files", data = "<file>")]
pub async fn upload(
	db: Db,
	mq: &State<Mq>,
	root: &State<FileDirectory>,
	id: i64,
	file: Form<TempFile<'_>>
) -> Result<Created<Json<DocumentFile>>, (Status, Json<ApiError>)> {
	let new_document_file = match 
		create_file(id, file.into_inner(), &root.0)
		.await {
		Ok(f) => f,
		Err(e) => return Err(e),
	};

	let created_file = match db
		.run(move |c| {
			diesel::insert_into(document_file::table)
				.values(new_document_file)
				.get_result::<DocumentFile>(c)
		})
		.await
		.map_err(|e| ApiError::internal_server_error(&e.to_string())) {
			Ok(d) => d,
			Err(e) => return Err(e)
	};

	if let Err(e) = mq.send(
		JobPayload {
			task: vec![Task::ConvertToPng, Task::CreatePreviews, Task::Ocr],
			document_file_id: created_file.id
		}, 
		RoutingKeys::WorkerQueue)
		.await {
			return Err(ApiError::internal_server_error(&e.to_string())) 
	};

	Ok(Created::new(format!("documents/{}/files", id)).body(Json(created_file)))
}

#[delete("/documents/files/<file_id>")]
pub async fn delete(
	file_id: i64,
	mq: &State<Mq>,
) -> Result<NoContent, (Status, Json<ApiError>)> {
	if let Err(e) = mq.send(
		JobPayload {
			task: vec![Task::Delete],
			document_file_id: file_id
		}, 
		RoutingKeys::WorkerQueue)
		.await {
			return Err(ApiError::internal_server_error(&e.to_string())) 
	};

	Ok(NoContent)
}