use std::net::TcpStream;
use std::{error::Error, io::Write};

use crate::error;
use crate::{
  frame::{Frame, builder::FrameBuilder},
  mail::Mail,
  serde::Serde,
};

pub struct Stream {
  io: TcpStream,
}

unsafe impl Send for Stream {}
unsafe impl Sync for Stream {}

impl Stream {
  pub fn new(io: TcpStream) -> Self {
    Self { io }
  }
}

impl Stream {
  pub fn send_mail(&mut self, mail: Mail) -> Result<(), Box<dyn Error>> {
    let id = (0, 0, 0);
    let (kv, files) = mail.destruct();
    let fb = FrameBuilder::builder().id(id).attach_kv(kv);
    self.io.write(&fb.serialize())?;

    for f in files {
      let (kv, data) = f.destruct();
      let fb = FrameBuilder::builder().id(id).attach_kv(kv);
      self.io.write(&fb.serialize())?;

      let fb = FrameBuilder::builder().id(id).attach_raw_data(data);
      self.io.write(&fb.serialize())?;
    }

    self.io.write(&FrameBuilder::fin(id).serialize())?;
    Ok(())
  }

  pub fn recv_mail<'a>(&mut self) -> Result<Mail<'a>, error::Error> {
    let mailmeta = Frame::new(&mut self.io)?;
    if let crate::frame::FrameType::Kv(kv) = mailmeta.data {
      let _filedata = Frame::new(&mut self.io)?;
      
      todo!()
    } else {
      Err(error::Error::FrameErrorUnsupportedVersion)
    }
  }
}
