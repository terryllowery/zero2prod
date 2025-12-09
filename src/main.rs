use std::fmt::format;
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let _configuration = get_configuration().expect("Failed to read configuration.");
    let address = format!("127.0.0.1:{}", _configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener)?.await
}
