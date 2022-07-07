use std::fs;
use std::path::Path;

use diesel::prelude::*;
use rocket::data::ToByteUnit;
use rocket::fs::TempFile;
use rocket::{State, Data};
use rocket::http::Status;
use rocket::response::status::{NoContent, Created};
use rocket::serde::json::{Json, serde_json};
use rocket::form::Form;
use uuid::Uuid;

use crate::broker::Mq;
use crate::db::Db;
use crate::files::{FileDirectory, create_file};
use crate::models::document_file::{DocumentFile, NewDocumentFile, DocumentFileMessage};
use crate::errors::{ApiError};
use crate::schema::document_file;

use docrab_lib::{Task, RoutingKeys, JobPayload};

// #[get("/documents/<id>/files")]
// pub async fn fetch(
// 	db: Db,
// 	id: i64
// ) -> Result<Json<Vec<DocumentFile>>, Error> {
// 	db
// 		.run(move |c| document_file::table
// 			.filter(document_file::document_id.eq(id))
// 			.load(c)
// 		)
// 		.await
// 		.map(Json)
// 		.map_err(|e| {
// 			ApiError::not_found(Status::DatabaseError, &e.to_string())
// 		})
// }

// #[get("/documents/<id>/files/<file_id>")]
// pub async fn get(
// 	db: Db,
// 	id: i64,
// 	file_id: i64
// ) -> Result<Json<Vec<DocumentFile>>, Error> {
// 	db
// 		.run(move |c| document_file::table
// 			.filter(document_file::document_id.eq(id)
// 			.and(document_file::id.eq(file_id)))
// 			.load(c)
// 		)
// 		.await
// 		.map(Json)
// 		.map_err(|e| {
// 			ApiError::not_found(Status::DatabaseError, &e.to_string())
// 		})
// // }

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

// #[delete("/documents/<id>/files/<file_id>")]
// pub async fn delete(
// 	db: Db,
// 	id: i64,
// 	file_id: i64,
// 	mq: &State<Mq>,
// ) -> Result<NoContent, Error> {
// 	let file = match db
// 		.run(move |c|
// 			document_file::table
// 				.filter(document_file::document_id.eq(id))
// 				.filter(document_file::id.eq(file_id))
// 				.first(c)
// 		)
// 		.await
// 		.map(|d: DocumentFile| d)
// 		.map_err(|e| {
// 			ApiError::not_found(
// 				Status::DatabaseError, 
// 				&e.to_string()
// 			)
// 		}) {
// 			Ok(f) => f,
// 			Err(e) => return Err(e),
// 	};

// 	// match fs::remove_file(path) {
// 	// 	Ok(_) => (),
// 	// 	Err(e) => match e.kind() {
// 	// 		std::io::ErrorKind::NotFound => (),
// 	// 		_ => return Err(ApiError::server_error(
// 	// 			Status::FileSystemError, 
// 	// 			"file system error"
// 	// 		))
// 	// 	}
// 	// };

// 	// match db
// 	// 	.run(move |c| {
// 	// 		let affected = match diesel::delete(document_file::table
// 	// 			.filter(document_file::id.eq(file_id)))
// 	// 			.execute(c) {
// 	// 			Ok(r) => r,
// 	// 			Err(e) => return Err(ApiError::server_error(
// 	// 				Status::DatabaseError,
// 	// 				&e.to_string()
// 	// 			)),
// 	// 		};

// 	// 		match affected {
// 	// 			1 => Ok(()),
// 	// 			0 => return Err(ApiError::server_error(
// 	// 				Status::DatabaseError,
// 	// 				"not found"
// 	// 			)),
// 	// 			_ => return Err(ApiError::server_error(
// 	// 				Status::DatabaseError,
// 	// 				"that wasn't supposed to happen"
// 	// 			))
// 	// 		}

// 	// 	})
// 	// 	.await
// 	// 	.map(|_| NoContent)
// 	// 	.map_err(|e| e) {
// 	// 		Ok(_) => (),
// 	// 		Err(e) => return Err(e)
// 	// 	};

// 	let serialized_filename = match serde_json::to_string( &DocumentFileMessage { 
// 			filename: file.filename,
// 			jobs: vec!(Job::Delete.to_string())
// 		} ) {
// 			Ok(f) => f,
// 			Err(e) => return Err(ApiError::server_error(
// 						Status::ServerError,
// 						&e.to_string()
// 			))
// 	};

// 	mq.send(serialized_filename.as_bytes(), 
// 		RoutingKeys::WorkerQueue)
// 	.await
// 	.map(|_| NoContent)
// 	.map_err(|e| {
// 		ApiError::server_error(
// 			Status::MessageQueueError,
// 			&e.to_string()	
// 		)
// 	})
// }