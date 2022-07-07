use lapin::{ConnectionProperties, Connection};
use lapin::message::DeliveryResult;
use lapin::types::FieldTable;
use lapin::options::{BasicConsumeOptions, BasicAckOptions};
use docrab_lib::{RoutingKeys, Job, EnvironmentVariable, get_environment_variable};

mod tasks;

mod config;
use config::JobMessage;

#[tokio::main]
async fn main() {
	let options = ConnectionProperties::default()
		.with_executor(tokio_executor_trait::Tokio::current())
		.with_reactor(tokio_reactor_trait::Tokio);

	let connection = Connection::connect(
			&get_environment_variable(EnvironmentVariable::RabbitMQUrl),
			options
		)
		.await
		.unwrap();

	let channel = connection.create_channel().await.unwrap();

	let consumer = channel
		.basic_consume(
			RoutingKeys::WorkerQueue.to_string(), 
				"docrab-converter-worker", 
				BasicConsumeOptions::default(),
				FieldTable::default()
		)
		.await
		.unwrap();
	


	consumer.set_delegate(move |delivery: DeliveryResult| async move {
		let delivery = match delivery {
			Ok(Some(delivery)) => delivery,
			Ok(None) => {
				dbg!("Consumer got cancelled");
				return;
			},
			Err(error) => {
				dbg!("Failed to consume queue message: {}", error);
				return;
			}
		};

		let payload = match std::str::from_utf8(&delivery.data) {
			Ok(p) => p,
			Err(e) => {
				dbg!("Failed to convert message to text: {}", e);
				return; 
			}
		};

		let job_message: JobMessage = serde_json::from_str(payload,).unwrap();

		for job_str in job_message.jobs {
			let job = match Job::from_string(&job_str) {
				Ok(j) => j,
				Err(_) =>  {
					dbg!("Failed to parse job from message");
					break;
				}
			};

			match job {
				Job::Convert => {
					dbg!("Starting job: Converting file");
					match tasks::convert_file_task(&job_message.filename).await {
						Ok(_) => (),
						Err(e) => {
							dbg!("{}", e);
							return;
						}
					}
				},
				Job::Ocr => {
					dbg!("Starting job: OCR on file");
					match tasks::ocr_on_file(&job_message.filename).await {
						Ok(_) => (),
						Err(e) => {
							dbg!("{}", e);
							return;
						}
					}
				},
				Job::Delete => {
					dbg!("Starting job: Deleting file");
					match tasks::delete_file_task(&job_message.filename).await {
						Ok(_) => (),
						Err(e) => {
							dbg!("{}", e);
							return;
						}
					}
				}
			}
		}

		delivery
			.ack(BasicAckOptions::default())
			.await
			.expect("Failed to ack worker job message");
	});

	loop {}
}