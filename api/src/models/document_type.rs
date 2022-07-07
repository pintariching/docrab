use serde::{Serialize, Deserialize};
use validator::Validate;
use crate::schema::document_type;

#[derive(Queryable, Selectable, Serialize, Clone, Debug)]
#[diesel(table_name = document_type)]
pub struct DocumentType {
	pub id: i64,
	pub label: String
}

#[derive(Insertable, Validate, Deserialize)]
#[diesel(table_name = document_type)]
pub struct NewDocumentType {
	#[validate(
		required(),
		length(max = 100)
	)]
	pub label: Option<String>
}

#[derive(AsChangeset, Deserialize, Validate)]
#[diesel(table_name = document_type)]
pub struct UpdateDocumentType {
	#[validate(length(max = 100))]
	pub label: Option<String>
}