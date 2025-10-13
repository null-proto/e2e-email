use std::sync::Arc;

use crate::{bytes::RawBytes, kv::Kv};


pub struct Mail<'a> {
  from : RawBytes,
  to : RawBytes,
  title : RawBytes,
  meta : Kv<'a>,
  body : Box<[File<'a>]>
}

pub struct File<'a> {
  meta : Kv<'a>,
  data : Box<[u8]>
}

impl<'a> Mail<'a> {
  pub fn destruct(self) -> (Kv<'a>,Box<[File<'a>]>) {
    (self.meta,self.body)
  }
}

impl<'a> File<'a> {
  pub fn destruct(self) -> (Kv<'a>,Box<[u8]>) {
    (self.meta,self.data)
  }
}


