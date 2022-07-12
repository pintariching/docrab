use std::error::Error;
use std::sync::Once;
use std::{fs, env};
use std::path::Path;

use docrab_lib::{JobPayload, Task};
use magick_rust::{magick_wand_genesis, MagickWand};
use sqlx::{Postgres, Pool};

use crate::models::document_file::{self, DocumentFile};

static START: Once = Once::new();

pub async fn delegate_task(mut job_payload: JobPayload, pool: Pool<Postgres>) -> Result<(), String> {
	// Sort tasks by priority in case the order gets switched somehow
	job_payload.task.sort_by(|a, b| 
		a.task_to_priority()
		.cmp(&b.task_to_priority()));
	
	if job_payload.task.contains(&Task::ConvertToPng) {
		if let Err(e) = convert_file_task(job_payload, &pool).await {
			return Err(e);
		};
	}

	Ok(())
}

// pub async fn delete_file_task(filename: &str) -> Result<(), String> {
// 	let file_root = get_environment_variable(EnvironmentVariable::FileRoot);
// 	let dir_path = Path::new(&file_root).join(filename);

// 	// Remove the document directory

// 	match std::fs::remove_dir_all(dir_path) {
// 		Ok(_) => return Ok(()),
// 		Err(e) => {
// 			return Err(format!("Error deleting file {}. Reason: {}", filename, e.to_string()));
// 		}
// 	}
// }

pub async fn convert_file_task(job_payload: JobPayload, pool: &Pool<Postgres>) -> Result<(), String> {
	let file_root = match env::var("FILE_ROOT") {
		Ok(f) => f,
		Err(e) => return Err(e.to_string()),
	};

	let root_path = Path::new(&file_root);

	let document_file = match DocumentFile::get(job_payload.document_file_id, pool).await {
		Ok(d) => d,
		Err(e) => return Err(e),
	};

	let file_path = root_path.join(&document_file.filename).join(&document_file.filename);
	let file_path_str = match file_path.to_str() {
		Some(f) => f,
		None => return Err("Could not convert from file path (PathBuf) to string (&str)".to_owned()),
	};

	let mut png_file_path = file_path.clone();
	png_file_path.set_extension("png");

	let png_file_path_str = match png_file_path.to_str() {
		Some(f) => f,
		None => return Err("Could not convert from png file path (PathBuf) to string (&str)".to_owned()),
	};

	// Convert the pdf to png images and update the entry in the database to done

	START.call_once(|| {
		magick_wand_genesis();
	});

	let wand = MagickWand::new();

	dbg!("{}", file_path_str);

	if let Err(e) = wand.read_image(file_path_str) {
		dbg!(e);
		return Err(e.to_string());
	};

	if let Err(e) = wand.write_image(png_file_path_str) {
		dbg!(e);
		return Err(e.to_string());
	};

	Ok(())
}

pub async fn ocr_on_file(filename: &str) -> Result<(), String> {

	// Do OCR on the highest resolution image and save the content to a database

	Ok(())
}