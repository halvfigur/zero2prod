//! main.rs

use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::configuration;
use zero2prod::telemetry;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = telemetry::get_subscriber("zero2prod".into(), "info".into());

    telemetry::init_subscriber(subscriber);

    let configuration = configuration::get_configuration().expect("Failed to read configuration");

    let connection = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres");

    let host_port = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(host_port)?;

    zero2prod::startup::run(listener, connection)?.await
}
