use std::error::Error;
use tokio::net::TcpListener;
mod client;
mod datasave;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let address = "127.0.0.1:8080";
    let bind_adress = TcpListener::bind(address).await?;

    loop {
        let (socket_client, _adress_client) = bind_adress.accept().await?;

        tokio::spawn(async move {
            if let Err(error) = client::handle_client(socket_client).await {
                eprintln!("Err: {}", error)
            }
        });
    }
}