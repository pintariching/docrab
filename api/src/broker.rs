use lapin::{Connection, Channel, ConnectionProperties, Error, BasicProperties};
use lapin::options::{QueueDeclareOptions, BasicPublishOptions};
use lapin::types::FieldTable;

use rocket::fairing::AdHoc;
use std::env;
use dotenv::dotenv;

use docrab_lib::{RoutingKeys, JobPayload};

pub struct Mq(pub Channel);

impl Mq {
	pub async fn send(&self, payload: JobPayload, routing_key: RoutingKeys) -> Result<(), String> {
		let payload = match payload.to_payload() {
			Ok(p) => p,
			Err(e) => return Err(e)
		};
		
		let result = self.0.basic_publish(
			"",
			routing_key.to_string(),
			BasicPublishOptions::default(), 
			&payload,
			BasicProperties::default().with_content_type("application/json".into())
		).await;

		match result {
			Ok(_) => Ok(()),
			Err(e) => Err(e.to_string()),
		}
	}
}

pub fn stage() -> AdHoc {
	AdHoc::on_ignite("Lapin broker", |rocket| async {
		dotenv().ok();

		let conn = Connection::connect(
				&env::var("RABBITMQ_URL")
						.expect("RABBITMQ_URL environment variable is not set"),
			ConnectionProperties::default()
		)
		.await
		.unwrap();

		let channel = conn.create_channel()
			.await
			.unwrap();

		channel.queue_declare(
			RoutingKeys::WorkerQueue.to_string(), 
			QueueDeclareOptions::default(),
			FieldTable::default()
		).await.unwrap();

		rocket.manage(Mq(channel))
	})
}
