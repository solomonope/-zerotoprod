use std::net::TcpListener;

use zerotoprod::configuration::get_configurations;
use zerotoprod::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configurations().expect("failed to read configuartion");
    let address = format!("0.0.0.0:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener)?.await
}
