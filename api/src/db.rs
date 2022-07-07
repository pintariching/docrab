// use diesel::prelude::*;
// use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::env;
use dotenv::dotenv;
use rocket::figment::{value::{Map, Value}, util::map, Figment};
use rocket::{Rocket, Build, fairing::AdHoc};
use rocket_sync_db_pools::{diesel, database};

// pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

#[database("diesel_postgres_pool")]
pub struct Db(diesel::PgConnection);

pub async fn run_migrations(rocket: Rocket<Build>) -> Rocket<Build> {
	// let mut conn = Db::get_one(&rocket).await.expect("Error establishing database connection for migrations");
	// conn.run_pending_migrations(MIGRATIONS).unwrap();
	rocket
}

pub fn stage() -> AdHoc {
	AdHoc::on_ignite("SQLx Stage", |rocket| async {
		rocket
		.attach(Db::fairing())
		.attach(AdHoc::on_ignite("Diesel migrations", run_migrations))
	})
}

pub fn from_env() -> Figment {
	dotenv().ok();

	let database_url = env::var("ROCKET_DATABASE").expect("ROCKET_DATABASE environment variable not found");

	let database: Map<_, Value> = map! {
		"url" => database_url.into(),
		"pool_size" => 10.into()
	};
	
	rocket::Config::figment().merge(("databases", map!["diesel_postgres_pool" => database]))
}