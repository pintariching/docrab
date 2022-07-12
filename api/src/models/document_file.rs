use rocket::fs::TempFile;
use serde::Serialize;
use validator::Validate;

use crate::models::document::Document;
use crate::schema::document_file;
const MAX_ID: i64 = i64::MAX;

#[derive(Identifiable, Queryable, Associations, Serialize, Debug)]
#[diesel(belongs_to(Document, foreign_key = document_id))]
#[diesel(table_name = document_file)]
pub struct DocumentFile {
	pub id: i64,
	pub label: String,
	pub document_id: i64,
	pub version: String,
	pub filename: String
}

#[derive(Insertable, Validate)]
#[diesel(table_name = document_file)]
pub struct NewDocumentFile {
	#[validate(
		required(),
		length(max = 100)
	)]
	pub label: Option<String>,

	#[validate(
		required(),
		range(max = "MAX_ID")
	)]
	pub document_id: Option<i64>,

	#[validate(
		required(),
		length(max = 100)
	)]
	pub version: Option<String>,
	
	#[validate(
		required(),
		length(max = 100)
	)]
	pub filename: Option<String>
}

#[derive(FromForm)]
pub struct UploadForm<'f> {
	pub file: TempFile<'f>
}

#[derive(Serialize)]
pub struct DocumentFileMessage {
	pub filename: String,
	pub jobs: Vec<String>
}