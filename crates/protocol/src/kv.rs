use crate::{bytes::{Bytes, RawBytes}, error::Error, serde::Serde};
use std::{collections::HashMap, sync::Arc};

pub struct Kv<'a>(HashMap<Bytes<'a>, RawBytes>);

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
      } else { break; }

      if let Some(v) = value.get(a) {
        let key = RawBytes::new(value.clone(), c + 1, a);
        c += *v as usize + 1;
        let value = RawBytes::new(value.clone(), a + 1, c);
        map.insert(key.into(), value);
      } else {
        return Err(Error::InvalidFrame);
      }
    }

    Ok(Kv(map))
  }
}

impl<'a> Kv<'a> {
  pub fn get(&'a self, k: &'a str) -> Option<&'a str> {
    self.0.get(&k.into()).map(|i| i.try_str().ok() )?
  }
}

impl<'a> KvBuilder<'a> {
  pub fn insert(&mut self, key: &'a str, value: &'a str) {
    self.0.push((key, value));
  }
}

impl Serde for KvBuilder<'_> {
  fn serialize(&self) -> Vec<u8> {
    let mut buf = Vec::default();
    for (k,v) in &self.0 {
      buf.push(k.as_bytes().len() as u8);
      buf.extend_from_slice(k.as_bytes());
      buf.push(v.as_bytes().len() as u8);
      buf.extend_from_slice(v.as_bytes());
    }
    buf
  }
}
