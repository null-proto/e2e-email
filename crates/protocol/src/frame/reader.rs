use std::fs::read;
use std::io::Read;

use crate::error::Error;
use crate::error::Result;
use crate::serde::Deser;

pub struct FrameReader {
  version: u8,
  size: u32, // 24bit or 3 octans
  id: u32,
  ftype: u8,
  data: Box<[u8]>,
}

// impl Serde for FrameReader {
//   fn serialize<'a>(&'a self) -> Vec<u8> {
//     let mut v = Vec::with_capacity(self.size as usize + 5);
//
//     // version
//     v.push(self.version);
//
//     // size
//     v.push(self.size as u8);
//     v.push((self.size >> 8) as u8);
//     v.push((self.size >> 16) as u8);
//     // v.push((self.size >> 24) as u8); only 3 octans
//
//     // id
//     v.push(self.id as u8);
//     v.push((self.id >> 8) as u8);
//     v.push((self.id >> 16) as u8);
//     v.push((self.id >> 24) as u8);
//
//     // padding
//     v.push(0);
//     v.push(0);
//     v.push(0);
//
//     v.push(self.ftype);
//
//     v.extend_from_slice(&self.data);
//     v
//   }
// }



impl<'a> FrameReader {
  pub fn new<T>(io:&mut T) -> Result<Self> where T : Read {
    let mut hbuf = [0u8; 12];

    _ = io.read(&mut hbuf);

    let version =  *hbuf.get(0).ok_or(Error::InvalidFrame)?;

    let mut size =  *hbuf.get(1).ok_or(Error::InvalidFrame)? as u32;
    size = (size << 8) | *hbuf.get(2).ok_or(Error::InvalidFrame)?  as u32;
    size = (size << 8) | *hbuf.get(3).ok_or(Error::InvalidFrame)?  as u32;

    let mut id = *hbuf.get(4).ok_or(Error::InvalidFrame)? as u32;
    id =  id << 8 | *hbuf.get(5).ok_or(Error::InvalidFrame)? as u32;
    id =  id << 8 | *hbuf.get(6).ok_or(Error::InvalidFrame)? as u32;
    id =  id << 8 | *hbuf.get(7).ok_or(Error::InvalidFrame)? as u32;

    let ftype = *hbuf.get(11).ok_or(Error::InvalidFrame)?;

    let mut buf = Vec::with_capacity(size as usize);
    _ = io.read(&mut buf);

    Ok(Self {
      version,
      size,
      id,
      ftype,
      data : buf.into()
    })
  }
}

impl<'a> FrameReader {
  fn into_frame(self) {

  }
}
