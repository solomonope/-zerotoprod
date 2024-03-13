use std::net::TcpListener;

use zerotoprod::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("0.0.0.0:8080")?;
    run(listener)?.await
}
