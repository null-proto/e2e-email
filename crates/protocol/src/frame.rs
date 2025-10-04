use crate::serde::{Deser, Serde};

pub struct Frame {
  version: u8,
  size: u32,
  id: u8,
  data: Box<[u8]>,
}

impl Serde for Frame {
  fn serialize<'a>(&'a self) -> Vec<u8> {
    let mut v = Vec::with_capacity(self.size as usize + 5);
    v.push(self.version);
    v.push(self.size as u8);
    v.push((self.size >> 8) as u8);
    v.push((self.size >> 16) as u8);
    v.push((self.size >> 24) as u8);
    v.push(self.id);
    v.extend_from_slice(&self.data);
    v
  }
}

impl<'a> Deser<'a> for Frame {
  type Type = Self;
  fn deserialize(data: &'a [u8]) -> crate::error::Result<Self::Type> {
    Ok(Self {
      version: data[0],
      size: ((data[1] as u32) << data[2]) << data[3],
      id: data[4],
      data: data[5..].into(),
    })
  }
}
