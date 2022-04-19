use tokio::io::BufReader;
use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use tokio::io::AsyncBufReadExt;

pub async fn process_session(stream: &mut TcpStream) -> std::io::Result<()> {
    let mut reader = BufReader::new(stream);

    loop {
        let mut ahaum = String::new();
        reader.read_line(&mut ahaum).await?;
        reader.write_all(b"+PONG\r\n").await?;
        // really need to commit this ?
    }
}