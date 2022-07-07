use rocket::http::Method;
use rocket_cors::{Cors, AllowedOrigins, AllowedHeaders};

pub fn get_cors() -> Cors {
	let allowed_origins = AllowedOrigins::all();

	let cors = rocket_cors::CorsOptions {
		allowed_origins,
		allowed_methods: vec![Method::Get, Method::Patch, Method::Post, Method::Delete].into_iter().map(From::from).collect(),
		allowed_headers: AllowedHeaders::All,
        allow_credentials: false,
		..Default::default()
	}
	.to_cors().unwrap();

	cors
}