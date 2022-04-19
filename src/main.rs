#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::fs;
use tokio::net::TcpListener;
use std::error::Error;

pub mod process;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    let mut listener = TcpListener::bind("127.0.0.1:6379").await?;

    loop {
        let (mut stream, addr) = listener.accept().await?;
        println!("New connection from {}", addr);

        tokio::spawn(async move {
            let finished = Some(process::process_session(&mut stream).await.err());
            println!("Close connection with {} because {:?}", addr, finished);
        });
    }
}


