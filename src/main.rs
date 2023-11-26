extern crate tokio;

use tokio::net::UdpSocket;
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {

    let socket = UdpSocket::bind("0.0.0.0:50513").await?;
    let mut buffer = [0; 1204];

    loop {
        let (len, addr) = socket.recv_from(&mut buffer).await?;
        println!("{:?} bytes recv from {:?}", len, addr)

    }
}