use tokio::net::TcpStream;

use crate::{frame::builder::FrameBuilder, mail::{File, Mail}};

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


impl Stream {
  pub(crate) fn send_file(&mut self, file: File) {

  }

  pub fn send_mail(&mut self , mail : Mail) {
    let (kv , files) = mail.destruct();
    let fb = FrameBuilder
  }
}
