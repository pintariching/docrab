use std::{path::Path, fs};
use docrab_lib::{EnvironmentVariable, get_environment_variable};

pub async fn delete_file_task(filename: &str) -> Result<(), String> {
	let file_root = get_environment_variable(EnvironmentVariable::FileRoot);
	let dir_path = Path::new(&file_root).join(filename);

	// Remove the document directory

	match std::fs::remove_dir_all(dir_path) {
		Ok(_) => return Ok(()),
		Err(e) => {
			return Err(format!("Error deleting file {}. Reason: {}", filename, e.to_string()));
		}
	}
}

pub async fn convert_file_task(filename: &str) -> Result<(), String> {
	let file_root = get_environment_variable(EnvironmentVariable::FileRoot);
	let dir_path = Path::new(&file_root).join(filename);
	
	// Move the pdf file to a new directory
	if !dir_path.is_dir() {
		match fs::create_dir(dir_path) {
			Ok(_) => todo!(),
			Err(_) => todo!(),
		}
	}


	// Convert the pdf to png images and update the entry in the database to done

	Ok(())
}

pub async fn ocr_on_file(filename: &str) -> Result<(), String> {

	// Do OCR on the highest resolution image and save the content to a database

	Ok(())
}