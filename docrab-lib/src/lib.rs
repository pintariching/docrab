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