use crate::{bytes::RawBytes, frame::field, kv::Kv};

pub struct Mail<'a> {
  pub from: RawBytes,
  pub to: RawBytes,
  pub title: RawBytes,
  pub meta: Kv<'a>,
  pub body: Box<[File<'a>]>,
}

pub struct File<'a> {
  meta: Kv<'a>,
  data: Box<[u8]>,
}

impl<'a> Mail<'a> {
  pub fn destruct(self) -> (Kv<'a>, Box<[File<'a>]>) {
    (self.meta, self.body)
  }
}

impl<'a> File<'a> {
  pub fn destruct(self) -> (Kv<'a>, Box<[u8]>) {
    (self.meta, self.data)
  }
}

impl<'a, T> Into<File<'a>> for (Kv<'a>, T)
where
  T: Into<Box<[u8]>>,
{
  fn into(self) -> File<'a> {
    File {
      meta: self.0,
      data: self.1.into(),
    }
  }
}

impl<'a, T> Into<Mail<'a>> for (RawBytes , RawBytes , RawBytes, Kv<'a>, T)
where
  T: Into<Box<[File<'a>]>>,
{
  fn into(self) -> Mail<'a> {
    Mail {
      from : self.0,
      to : self.1,
      title : self.2,
      meta: self.3,
      body: self.4.into(),
    }
  }
}
