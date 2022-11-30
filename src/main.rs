//! main.rs

use secrecy::ExposeSecret;
use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;
use zero2prod::configuration;
use zero2prod::telemetry;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = telemetry::get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);

    telemetry::init_subscriber(subscriber);

    let configuration = configuration::get_configuration().expect("Failed to read configuration");

    let connection_pool = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy(&configuration.database.connection_string().expose_secret())
        .expect("Failed to create Postgres connection pool");

    let host_port = format!("127.0.0.1:{}", configuration.application.port);
    let listener = TcpListener::bind(host_port)?;

    zero2prod::startup::run(listener, connection_pool)?.await
}
