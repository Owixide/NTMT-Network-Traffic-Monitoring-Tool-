use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;

pub async fn save_info(datapacket: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = OpenOptions::new().create(true).append(true).open("saved_packets.txt").await?;

    file.write_all(datapacket).await?;
    file.write_all(b"\n---\n").await?;

    file.flush().await?;

    Ok(())
}
