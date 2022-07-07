
use rocket::{fairing::AdHoc, fs::TempFile, http::Status, serde::json::Json};
use dotenv::dotenv;
use uuid::Uuid;
use std::{env, path::{Path, PathBuf}, fs};

use crate::{errors::ApiError, models::document_file::NewDocumentFile};

pub struct FileDirectory(pub String);

pub fn stage() -> AdHoc {
	AdHoc::on_ignite("File directory root config", |rocket| async {
		dotenv().ok();

		let path = env::var("FILE_ROOT").expect("FILE_ROOT environment variable is not set");

		rocket.manage(FileDirectory(path))
	})
}

pub async fn create_file(document_id: i64, mut file: TempFile<'_>, file_directory: &str) -> Result<NewDocumentFile, (Status, Json<ApiError>)> {
	let root_path = Path::new(file_directory);
	let uuid = Uuid::new_v4().to_string();

	let filename = match file.name() {
		Some(f) => f.to_owned(),
		None => String::new(),
	};

	let filename_with_uuid = match file.name() {
		Some(n) => format!("{}-{}", n, String::from(&uuid)),
		None => String::from(&uuid)
	};

	let directory = match check_if_file_directory_exists(&filename_with_uuid, &uuid, root_path) {
		Ok(d) => d,
		Err(e) => return Err(e),
	};

	let file_path = directory.join(&filename_with_uuid);

	if let Err(e) = file.persist_to(file_path).await {
		return Err(ApiError::internal_server_error(&e.to_string()));
	}

	let document_file = NewDocumentFile {
		label: Some(filename),
		document_id: Some(document_id),
		version: Some(String::new()),
		filename: Some(filename_with_uuid)
	};

	Ok(document_file)
}

fn check_if_file_directory_exists(filename: &str, uuid: &str, root_path: &Path) -> Result<PathBuf, (Status, Json<ApiError>)> {
	let filename = Path::new(&filename);

	let file_stem = match filename.file_stem() {
		Some(f) => match f.to_str() { 
			Some(str) => str.to_owned(),
			None => uuid.to_string()
		},
		None => uuid.to_string(),
	};

	let directory_path = root_path.join(Path::new(&file_stem));

	if directory_path.is_dir() {
		return Ok(directory_path)
	} else {
		match fs::create_dir_all(&directory_path) {
			Ok(_) => return Ok(directory_path),
			Err(e) => return Err(ApiError::internal_server_error(&e.to_string())),
		}
	}
}