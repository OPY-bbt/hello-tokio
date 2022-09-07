#![warn(rust_2018_idioms)]

use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

use std::error::Error;
use std::{thread, time};

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    // Open a TCP stream to the socket address.
    //
    // Note that this is the Tokio TcpStream, which is fully async.
    let mut stream = TcpStream::connect("127.0.0.1:4000").await?;
    println!("created stream");

    let ten_millis = time::Duration::from_millis(5000);
    thread::sleep(ten_millis);

    let result = stream.write(b"hello world\n").await;
    println!("wrote to stream; success={:?}", result.is_ok());

    let ten_millis = time::Duration::from_millis(5000);
    thread::sleep(ten_millis);

    Ok(())
}