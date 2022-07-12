use errors::ApiError;
use rocket::{serde::json::Json, http::Status};

#[macro_use] extern crate rocket;
#[macro_use] extern crate lazy_static;
// #[macro_use] extern crate diesel_migrations;
#[macro_use] extern crate diesel;

mod db;
mod errors;
mod routes;
mod models;
mod broker;
mod schema;
mod files;

use crate::routes::{
	document_file,
	document_type,
	document_tag,
	document,
	tag,
};

#[catch(404)]
fn not_found() -> (Status, Json<ApiError>) {
	ApiError::not_found("no matching routes found")
}

#[launch]
pub fn rocket() -> _ {
	rocket::custom(db::from_env())
		.attach(db::stage())
		.attach(broker::stage())
		.attach(files::stage())
		.register("/", catchers![not_found])
		.mount(
			"/api",
			routes![
				tag::fetch,
				tag::get,
				tag::create,
				tag::update,
				tag::delete,
				document_type::fetch,
				document_type::get,
				document_type::create,
				document_type::update,
				document_type::delete,
				document::fetch,
				document::get,
				document::create,
				document::update,
				document::delete,
				document_tag::fetch,
				document_tag::attach,
				document_tag::attach_multiple,
				document_tag::remove,
				document_file::fetch,
				document_file::get,
				document_file::upload,
				document_file::delete
			]
		)		
}
