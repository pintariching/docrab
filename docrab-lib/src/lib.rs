use std::path::PathBuf;

use dotenv::dotenv;

pub enum RoutingKeys {
	WorkerQueue
}

impl RoutingKeys {
	pub fn to_string(&self) -> &'static str {
		match self {
			RoutingKeys::WorkerQueue => "docrab-workers",
		}
	}
}

pub enum Job {
	Convert,
	Ocr,
	Delete,
}

impl Job {
	pub fn to_string(&self) -> String {
		match self {
			Job::Convert => String::from("CONVERT"),
			Job::Ocr => String::from("OCR"),
			Job::Delete => String::from("DELETE"),
		}
	}

	pub fn from_string(input: &str) -> Result<Job, ()> {
		match input {
			"CONVERT" => Ok(Job::Convert),
			"OCR" => Ok(Job::Ocr),
			"DELETE" => Ok(Job::Delete),
			_ => Err(())
		}
	}
}

pub enum EnvironmentVariable {
	DatabaseUrl,
	RabbitMQUrl,
	FileRoot
}

pub fn get_environment_variable(variable: EnvironmentVariable) -> String {
	dotenv().ok();

	match variable {
		EnvironmentVariable::DatabaseUrl => std::env::var("DATABASE_URL")
			.expect("DATABASE_URL environment variable is not set"),
		EnvironmentVariable::RabbitMQUrl => std::env::var("RABBITMQ_URL")
			.expect("RABBITMQ_URL environment variable is not set"),
		EnvironmentVariable::FileRoot => std::env::var("FILE_ROOT")
			.expect("FILE_ROOT environment variable is not set"),
	}
}