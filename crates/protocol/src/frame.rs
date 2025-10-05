use crate::{error::Error, serde::{Deser, Serde}};

pub struct Frame {
  version: u8,
  size: u32, // 24bit or 3 octans
  id: u32,
  data: Box<[u8]>,
}

impl Serde for Frame {
  fn serialize<'a>(&'a self) -> Vec<u8> {
    let mut v = Vec::with_capacity(self.size as usize + 5);

    // version
    v.push(self.version);

    // size
    v.push(self.size as u8);
    v.push((self.size >> 8) as u8);
    v.push((self.size >> 16) as u8);
    // v.push((self.size >> 24) as u8); only 3 octans
    
    // id
    v.push(self.id as u8);
    v.push((self.id >> 8) as u8);
    v.push((self.id >> 16) as u8);
    v.push((self.id >> 24) as u8);

    // padding
    v.push(0);
    v.push(0);
    v.push(0);
    v.extend_from_slice(&self.data);
    v
  }
}

impl<'a> Deser<'a> for Frame {
  type Type = Self;
  fn deserialize(data: &'a [u8]) -> crate::error::Result<Self::Type> {

    let version =  *data.get(0).ok_or(Error::InvalidFrame)?;

    let mut size =  *data.get(1).ok_or(Error::InvalidFrame)? as u32;
    size = (size << 8) | *data.get(2).ok_or(Error::InvalidFrame)?  as u32;
    size = (size << 8) | *data.get(3).ok_or(Error::InvalidFrame)?  as u32;

    let mut id = *data.get(4).ok_or(Error::InvalidFrame)? as u32;
    id =  id << 8 | *data.get(5).ok_or(Error::InvalidFrame)? as u32;
    id =  id << 8 | *data.get(6).ok_or(Error::InvalidFrame)? as u32;
    id =  id << 8 | *data.get(7).ok_or(Error::InvalidFrame)? as u32;


    Ok(Self {
      version,
      size,
      id,
      data: data.get(11..(size as usize)+8 ).ok_or(Error::InvalidFrame)?.into(),
    })
  }
}
