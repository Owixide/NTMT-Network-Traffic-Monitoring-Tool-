use crate::datasave;
use std::error::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

pub async fn handle_client(mut socket: TcpStream) -> Result<(), Box<dyn Error>> {
    let mut buffer: Vec<u8> = Vec::with_capacity(4096);
    let mut temp_buffer = [0_u8; 4096];

    loop {
        let bytes_read = socket.read(&mut temp_buffer).await?;

        if bytes_read == 0 {
            break;
        }
        buffer.extend_from_slice(&temp_buffer[..bytes_read]);
    }

    if buffer.is_empty() {
        return Ok(());
    }

    datasave::save_info(&buffer).await?;

    let data_str = String::from_utf8_lossy(&buffer);
    let mut target_address = String::new();
    let mut host_found = false;

    for line in data_str.lines() {
        if let Some(strip) = line.strip_prefix("Host:") {
            let host = strip.trim().to_string();
            target_address = if host.contains(':') {
                host
            } else {
                format!("{}:80", host)
            };

            host_found = true;
            break;
        }
    }

    if !host_found {
        return Err("Connect error".into());
    }

    let mut stream = TcpStream::connect(&target_address).await?;
    stream.write_all(&buffer).await?;

    let mut answer_buffer: Vec<u8> = Vec::with_capacity(4096);
    let mut temp_answer = [0_u8; 4096];

    loop {
        let bytes_read = stream.read(&mut temp_answer).await?;
        if bytes_read == 0 {
            break;
        }
        answer_buffer.extend_from_slice(&temp_answer[..bytes_read]);
    }

    socket.write_all(&answer_buffer).await?;
    datasave::save_info(&answer_buffer).await?;

    socket.shutdown().await?;
    stream.shutdown().await?;

    println!("Connect: ({:?} bytes (SAVED))", answer_buffer.len());
    Ok(())
}
