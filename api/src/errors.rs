use rocket::{serde::json::Json, http::Status};
use serde::Serialize;
use validator::{ValidationErrors, ValidationError};
use convert_case::{Case, Casing};

#[derive(Serialize, Debug)]
pub struct ApiError {
	pub status: String,
	pub reason: String,
	pub validation_errors: Option<Vec<FieldError>>,
}

#[derive(Serialize, Debug)]
pub struct FieldError {
	pub field: String,
	pub errors: Vec<String>,
}

impl FieldError {
	fn from_validation_error(field: &str, errors: &[ValidationError]) -> FieldError {
		FieldError {
			field: field.from_case(Case::Snake).to_case(Case::Title),
			errors: errors.iter()
				.map(|e| if let Some(message) = e.message.clone() {
					String::from(message)
				} else {
					String::new()
				})
				.collect()
		}
	}
}

impl ApiError {
	pub fn server_error(status: Status, reason: &str) -> (Status, Json<ApiError>) {
		(status, Json(ApiError {
			status: status.to_string(),
			reason: reason.to_owned(),
			validation_errors: None
		}))
	}

	pub fn internal_server_error(reason: &str) -> (Status, Json<ApiError>) {
		(Status::InternalServerError, 
		Json(ApiError {
			status: Status::InternalServerError.to_string(),
			reason: reason.to_owned(),
			validation_errors: None
		}))
	}

	pub fn not_found(reason: &str) -> (Status, Json<ApiError>) {
		(Status::NotFound, Json(ApiError {
			status: Status::NotFound.to_string(),
			reason: reason.to_owned(),
			validation_errors: None,
		}))
	}

	pub fn validation_error(validation_errors: ValidationErrors) -> (Status, Json<ApiError>) {
		return (Status::BadRequest, Json(ApiError {
			status: Status::BadRequest.to_string(),
			reason: String::new(),
			validation_errors: Some(validation_errors
				.field_errors()
				.into_iter()
				.map(|field| FieldError::from_validation_error(field.0, field.1))
				.collect()
			)
		}))
	}
}