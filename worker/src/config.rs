use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct JobMessage {
	pub filename: String,
	pub jobs: Vec<String>
}