//! main.rs

use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::configuration;

#[tokio::main]
async fn main() -> std::io::Result<()> {
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
