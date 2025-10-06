use tokio::net::TcpStream;

pub struct Stream {
  io : TcpStream,
}

unsafe impl Send for Stream {}
unsafe impl Sync for Stream {}

impl Stream {
  pub fn new(io : TcpStream) -> Self {
    Self { io }
  }
}

