use regex::Regex;
use serde::{Serialize, Deserialize};
use validator::Validate;
use crate::schema::tag;

lazy_static! {
	// regex from https://mkyong.com/regular-expressions/how-to-validate-hex-color-code-with-regular-expression/
    static ref RE_HEX_COLOR: Regex = Regex::new(r"^#(?:[0-9a-fA-F]{6})$").unwrap();
}

#[derive(Queryable, Identifiable, Serialize, Debug)]
#[diesel(table_name = tag)]
pub struct Tag {
	pub id: i64,
	pub label: String,
	pub color: String
}

#[derive(Insertable, Validate, Deserialize)]
#[diesel(table_name = tag)]
pub struct NewTag {
	#[validate(
		required(),
		length(max = 100)
	)]
	pub label: Option<String>,

	#[validate(
		required(),
		regex(path = "RE_HEX_COLOR", message = "invalid color format")
	)]
	pub color: Option<String>
}

#[derive(AsChangeset, Deserialize, Validate)]
#[diesel(table_name = tag)]
pub struct UpdateTag {
	#[validate(
		length(max = 100)
	)]
	pub label: Option<String>,

	#[validate(
		length(equal = 7),
		regex(path = "RE_HEX_COLOR", message = "invalid color format")
	)]
	pub color: Option<String>
}

#[test]
fn test_correct_new_tag_color_regex() {
	let tag = NewTag {
		label: Some("test".to_string()),
		color: Some("#ffffff".to_string())
	};

	assert!(tag.validate().is_ok());
}

#[test]
fn test_incorrect_new_tag_color_regex() {
	let tag = NewTag {
		label: Some("test".to_string()),
		color: Some("ddddddd".to_string())
	};

	assert!(tag.validate().is_err());
}

#[test]
fn test_correct_update_tag_color_regex() {
	let tag = UpdateTag {
		label: None,
		color: Some("#ffffff".to_string())
	};

	assert!(tag.validate().is_ok());
}

#[test]
fn test_incorrect_update_tag_color_regex() {
	let tag = UpdateTag {
		label: None,
		color: Some("ddddddd".to_string())
	};

	assert!(tag.validate().is_err());
}