
use rocket::fairing::AdHoc;
use dotenv::dotenv;
use std::env;

pub struct FileDir(pub String);

pub fn stage() -> AdHoc {
	AdHoc::on_ignite("File directory root config", |rocket| async {
		dotenv().ok();

		let path = env::var("FILE_ROOT").expect("FILE_ROOT environment variable is not set");

		rocket.manage(FileDir(path))
	})
}