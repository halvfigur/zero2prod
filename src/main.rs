//! main.rs

use env_logger::Env;
use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::configuration;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let configuration = configuration::get_configuration().expect("Failed to read configuration");

    //let listener = TcpListener::bind("127.0.0.1:8000")?;
    let host_port = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(host_port)?;
    //let connection = PgConnection::connect(&configuration.database.connection_string())
    let connection = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres");

    zero2prod::startup::run(listener, connection)?.await
}
