use serde::{Serialize, Deserialize};

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

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
pub struct File {
	pub file: Vec<u8>,
	pub file_name: String,
	pub file_size: usize,
	pub uuid: String
}

#[derive(Serialize, Deserialize)]
pub struct JobPayload {
	pub job: Job,
	pub file: File
}

impl JobPayload {
	pub fn to_payload(&self) -> Result<Vec<u8>, String> {
		match serde_json::to_string(self) {
			Ok(s) => return Ok(s.as_bytes().to_vec()),
			Err(e) => return Err(e.to_string())
		}
	}

	pub fn from_payload(bytes: &[u8]) -> Result<JobPayload, String> {
		match serde_json::from_slice(bytes) {
			Ok(job_payload) => Ok(job_payload),
			Err(e) => Err(e.to_string())
		}
	}
}