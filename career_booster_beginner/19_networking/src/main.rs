use std::io::{ Read, Result, Write };
use std::net::UdpSocket;

// Rust provides networking primitives in std::net. TCP and UDP will be introduced
// 1) TCP
fn createServer() -> Result<()> {
  let listener = std::net::TcpListener::bind("0.0.0.0:12345")?;

  // This single threaded server can handle only one incoming connection at a
  // time.
  for stream in listener.incoming() {
    let mut stream = stream?;
    let mut buffer = [0u8; 4096];
    let count = stream.read(&mut buffer)?;
    stream.write_all(&buffer[0..count])?;
  }
  Ok(())
}

fn useClient() -> Result<()> {
  // And this is how you write and read data over a TcpStream (as a client)
  let mut stream = std::net::TcpStream::connect("127.0.0.1:12345")?;

  stream.write_all(&[0, 1, 2, 3])?;

  let mut buffer = [0u8; 4];
  stream.read_exact(&mut buffer)?;
  println!("Received {buffer:?}");

  Ok(())
}

// 2) UDP
// If you want to use the User Datagram Protocol, which provides no guarantees about the order in which frames are sent nor guarantees delivery, you can use the 👉 UdpSocket.

fn main() -> Result<()> {}
