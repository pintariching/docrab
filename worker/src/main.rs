use std::borrow::Borrow;
use std::env;
use std::os::unix::thread;
use std::sync::{Arc, Mutex};
use dotenvy::dotenv;

use lapin::{ConnectionProperties};
use lapin::message::DeliveryResult;
use lapin::types::FieldTable;
use lapin::options::{BasicConsumeOptions, BasicAckOptions};
use sqlx::Acquire;
use sqlx::postgres::PgPoolOptions;

use docrab_lib::{JobPayload, RoutingKeys};
use tasks::delegate_task;

mod tasks;
mod models;

#[tokio::main]
async fn main() {
	let options = ConnectionProperties::default()
		.with_executor(tokio_executor_trait::Tokio::current())
		.with_reactor(tokio_reactor_trait::Tokio);

	dotenv().ok();

	let connection = lapin::Connection::connect(
			&env::var("RABBITMQ_URL").expect("RABBITMQ_URL environment variable is not set"),
			options
		)
		.await
		.unwrap();

	let channel = connection.create_channel().await.unwrap();

	let consumer = channel
		.basic_consume(
			RoutingKeys::WorkerQueue.to_string(), 
				"docrab-worker", 
				BasicConsumeOptions::default(),
				FieldTable::default()
		)
		.await
		.unwrap();

	let pool = PgPoolOptions::new()
		.max_connections(5)
		.connect(&env::var("DATABASE_URL").expect("DATABASE_URL environment variable is not set"))
		.await
		.unwrap();

	consumer.set_delegate(move |delivery: DeliveryResult| {
		dbg!("Cloning pool");
		let pool = pool.clone();
		 async move {
			dbg!("Starting task");

			let delivery = match delivery {
				Ok(Some(delivery)) => delivery,
				Ok(None) => {
					// log error
					dbg!("Consumer got cancelled");
					return;
				},
				Err(error) => {
					// log error
					dbg!("Failed to consume queue message: {}", error);
					return;
				}
			};

			dbg!("Got a delivery");

			let job_payload = match JobPayload::from_payload(&delivery.data) {
				Ok(payload) => payload,
				Err(e) => {
					// log error
					dbg!("Error deserializing payload");
					return;
				}
			};

			dbg!("Deserialized payload");

			if let Err(e) = delegate_task(job_payload, pool).await {
				// log error
				dbg!(e);
				dbg!("Error when delegating task");
				return;
			};
			
			dbg!("Delegated task");

			if let Err(e) = delivery
				.ack(BasicAckOptions::default())
				.await {
				// log error
				dbg!("Failed to ack message");
				return;
			};

			dbg!("Acknowledged messaage");
		}
	});
	loop {}
}