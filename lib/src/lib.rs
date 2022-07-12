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
pub enum Task {
	ConvertToPng,
	CreatePreviews,
	Ocr,
	Delete,
}

impl Task {
	pub fn to_string(&self) -> String {
		match self {
			Task::ConvertToPng => String::from("CONVERT_TO_PNG"),
			Task::CreatePreviews => String::from("CREATE_PREVIEWS"),
			Task::Ocr => String::from("OCR"),
			Task::Delete => String::from("DELETE"),
		}
	}

	pub fn from_string(input: &str) -> Result<Task, ()> {
		match input {
			"CONVERT_TO_PNG" => Ok(Task::ConvertToPng),
			"CREATE_PREVIEWS" => Ok(Task::CreatePreviews),
			"OCR" => Ok(Task::Ocr),
			"DELETE" => Ok(Task::Delete),
			_ => Err(())
		}
	}

	pub fn task_to_priority(&self) -> u64 {
		match self {
			Task::ConvertToPng => 0,
			Task::CreatePreviews => 1,
			Task::Ocr => 2,
			Task::Delete => 10,
		}
	}
}

#[derive(Serialize, Deserialize)]
pub struct JobPayload {
	pub task: Vec<Task>,
	pub document_file_id: i64
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