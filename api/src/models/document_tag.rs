use serde::{Serialize, Deserialize};
use validator::Validate;

use crate::models::{
	document::Document,
	tag::Tag
};

use crate::schema::document_tag;

const MAX_ID: i64 = i64::MAX;

#[derive(Identifiable, Queryable, Associations, Serialize, Debug)]
#[diesel(belongs_to(Document, foreign_key = document_id))]
#[diesel(belongs_to(Tag))]
#[diesel(table_name = document_tag)]
pub struct DocumentTag {
	pub id: i64,
	pub tag_id: i64,
	pub document_id: i64,
}

#[derive(Validate, Insertable, Deserialize, Clone)]
#[diesel(table_name = document_tag)]
pub struct NewDocumentTag {
	#[validate(
		required(),
		range(max = "MAX_ID")
	)]
	pub tag_id: Option<i64>,

	pub document_id: Option<i64>
}