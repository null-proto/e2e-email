use crate::{
  bytes::{Bytes, RawBytes},
  error::Error,
  serde::Serde,
};
use std::{collections::HashMap, sync::Arc};

pub struct Kv<'a>(HashMap<Bytes<'a>, RawBytes>, Arc<[u8]>);

pub struct KvBuilder<'a>(Vec<(&'a str, &'a str)>);

impl TryFrom<&[u8]> for Kv<'_> {
  type Error = crate::error::Error;
  fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
    let mut map = HashMap::default();
    let value: Arc<[u8]> = Arc::from(value);
    let mut c = 0usize;
    let mut a = 0usize;
    while c < value.len() {
      if let Some(v) = value.get(c) {
        a += *v as usize + 1;
      } else {
        break;
      }

      if let Some(v) = value.get(a) {
        let key = RawBytes::new(value.clone(), c + 1, a);
        c += *v as usize + 1;
        let value = RawBytes::new(value.clone(), a + 1, c);
        map.insert(key.into(), value);
      } else {
        return Err(Error::InvalidFrame);
      }
    }
    Ok(Kv(map, value.clone()))
  }
}

impl TryFrom<Box<[u8]>> for Kv<'_> {
  type Error = crate::error::Error;
  fn try_from(value: Box<[u8]>) -> Result<Self, Self::Error> {
    let mut map = HashMap::default();
    let value: Arc<[u8]> = Arc::from(value);
    let mut c = 0usize;
    let mut a = 0usize;
    while c < value.len() {
      match value.get(c) {
        Some(n) => {
          a += 1 + *n as usize;
        }
        None => break,
      }

      match value.get(a) {
        Some(n) => {
          let key = RawBytes::new(value.clone(), c + 1, a);
          c = 1 + a + *n as usize;
          let value = RawBytes::new(value.clone(), a + 1, c);
          map.insert(key.into(), value);
          a = c;
        }
        None => {
          return Err(Error::InvalidFrame);
        }
      }
    }
    Ok(Kv(map, value.clone()))
  }
}

impl TryFrom<Vec<u8>> for Kv<'_> {
  type Error = crate::error::Error;
  fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
    let mut map = HashMap::default();
    let value: Arc<[u8]> = Arc::from(value);
    let mut c = 0usize;
    let mut a = 0usize;
    while c < value.len() {
      match value.get(c) {
        Some(n) => {
          a += 1 + *n as usize;
        }
        None => break,
      }

      match value.get(a) {
        Some(n) => {
          let key = RawBytes::new(value.clone(), c + 1, a);
          c = 1 + a + *n as usize;
          let value = RawBytes::new(value.clone(), a + 1, c);
          map.insert(key.into(), value);
          a = c;
        }
        None => {
          return Err(Error::InvalidFrame);
        }
      }
    }
    Ok(Kv(map, value.clone()))
  }
}

impl<'a> Kv<'a> {
  pub fn get(&'a self, k: &'a str) -> Option<&'a str> {
    self.0.get(&k.into()).map(|i| i.try_str().ok())?
  }

  pub fn get_raw(&'a self, k: &'a str) -> Option<RawBytes> {
    self.0.get(&k.into()).map(|i| i.to_owned())
  }

  pub fn len(&self) -> usize {
    self.0.len()
  }

  pub fn build() -> KvBuilder<'a> {
    KvBuilder(Default::default())
  }
}

impl<'a> KvBuilder<'a> {
  pub fn insert(mut self, key: &'a str, value: &'a str) -> Self {
    self.0.push((key, value));
    self
  }

  pub fn finish(self) -> Kv<'a> {
    let mut vec: Vec<u8> = Vec::new();
    for i in self.0 {
      let k = i.0.as_bytes();
      let v = i.1.as_bytes();
      vec.push(k.len() as u8);
      vec.extend_from_slice(k);
      vec.push(v.len() as u8);
      vec.extend_from_slice(v);
    }
    Kv::try_from(vec).unwrap()
  }
}

impl Serde for Kv<'_> {
  fn serialize(&self) -> Vec<u8> {
    self.1.to_vec()
  }
}

impl Serde for KvBuilder<'_> {
  fn serialize(&self) -> Vec<u8> {
    let mut buf = Vec::default();
    for (k, v) in &self.0 {
      let k = k.as_bytes();
      let v = v.as_bytes();
      buf.push(k.len() as u8);
      buf.extend_from_slice(k);
      buf.push(v.len() as u8);
      buf.extend_from_slice(v);
    }
    buf
  }
}

#[cfg(test)]
mod unitest {
  use crate::frame::field::*;
  use crate::kv::Kv;

  #[test]
  fn kv_builder() {
    let kv = Kv::build()
      .insert(M_FROM, "me@me.me")
      .insert(M_TO, "you@you.you")
      .finish();

    assert_eq!(kv.get(M_FROM).unwrap(), "me@me.me");
    assert_eq!(kv.get(M_TO).unwrap(), "you@you.you");
  }
}
