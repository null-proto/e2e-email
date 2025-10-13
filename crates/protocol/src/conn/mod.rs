use crate::error;
use crate::error::Result;
use crate::frame::Frame;
use crate::frame::builder::FrameBuilder;
use crate::frame::{FrameType, field};
use crate::mail::File;
use crate::mail::Mail;
use crate::serde::Serde;
use std::io::Read;
use std::io::Write;

pub struct Stream<T>
where
  T: Read + Write,
{
  io: T,
}

unsafe impl<T: Read + Write> Send for Stream<T> {}
unsafe impl<T: Read + Write> Sync for Stream<T> {}

impl<T> Stream<T>
where
  T: Read + Write,
{
  pub fn new(io: T) -> Self {
    Self { io }
  }
}

impl<T: Read + Write> Stream<T> {
  pub fn send_mail(&mut self, mail: Mail) -> Result<()> {
    let id = (0, 0, 0);
    let (kv, files) = mail.destruct();
    let fb = FrameBuilder::builder().id(id).attach_kv(kv);
    _ = self.io.write(&fb.serialize());

    for f in files {
      let (kv, data) = f.destruct();
      let fb = FrameBuilder::builder().id(id).attach_kv(kv);
      _ = self.io.write(&fb.serialize());

      let fb = FrameBuilder::builder().id(id).attach_raw_data(data);
      _ = self.io.write(&fb.serialize());
    }

    _ = self.io.write(&FrameBuilder::fin(id).serialize());
    Ok(())
  }

  fn get_file<'a>(&mut self) -> Result<File<'a>> {
    if let FrameType::Kv(meta) = Frame::new(&mut self.io)?.data {
      if let FrameType::Raw(data) = Frame::new(&mut self.io)?.data {
        Ok((meta, data).into())
      } else {
        Err(error::Error::InvalidFrame)
      }
    } else {
      Err(error::Error::InvalidFrame)
    }
  }

  pub fn recv_mail<'a>(&mut self) -> Result<Mail<'a>> {
    let mailmeta = Frame::new(&mut self.io)?;
    let mut files: Vec<File> = vec![];
    if let crate::frame::FrameType::Kv(kv) = mailmeta.data {
      // let file_count = kv.get(field::M_FILE_COUNT).unwrap().parse::<usize>().unwrap();

      let from = kv.get_raw(field::M_FROM).unwrap();
      let to = kv.get_raw(field::M_TO).unwrap();
      let title = kv.get_raw(field::M_TITLE).unwrap();

      loop {
        match self.get_file() {
          Ok(file) => files.push(file),
          Err(_) => break,
        }
      }

      Ok((from, to, title, kv, files).into())
    } else {
      Err(error::Error::FrameErrorUnsupportedVersion)
    }
  }
}
