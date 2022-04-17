use tokio::io::BufReader;
use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use tokio::io::AsyncBufReadExt;

pub async fn process_session(stream: &mut TcpStream) -> std::io::Result<()> {
    let mut stream = BufReader::new(stream);

    let mut result = String::new();
    stream.read_line(&mut result).await?;
    println!("Recieved {}", result);

    stream.write_all(b"PONG").await?;
    Ok(())
}