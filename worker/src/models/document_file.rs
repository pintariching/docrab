use sqlx::{Pool, Postgres};

pub struct DocumentFile {
	pub id: i64,
	pub label: String,
	pub document_id: i64,
	pub version: String,
	pub filename: String
}

impl DocumentFile {
	pub async fn get(id: i64, pool: &Pool<Postgres>) -> Result<DocumentFile, String> {
		let document_file = match sqlx::query_as!(
				DocumentFile,
				"SELECT id, label, document_id, version, filename FROM document_file WHERE id = $1",
				id
			)
			.fetch_one(pool)
			.await
			.map_err(|e| e.to_string()) {
			Ok(row) => row,
			Err(e) => return Err(e),
		};

		Ok(document_file)
	}
}