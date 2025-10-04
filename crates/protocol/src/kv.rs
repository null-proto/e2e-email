use crate::bytes::RawBytes;
use std::{collections::HashMap, sync::Arc};

pub struct Kv(HashMap<RawBytes, RawBytes>);

pub struct KvBuilder<'a>(Vec<(&'a str, &'a str)>);

impl TryFrom<&[u8]> for Kv {
  type Error = crate::error::Error;
  fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
    let mut map = HashMap::default();
    let value: Arc<[u8]> = Arc::from(value);

    let mut c = 0usize;
    let mut a = 0usize;

    while c < value.len() {
      a += value[c] as usize + 1;
      let key = RawBytes::new(value.clone(), c + 1, a);
      c += value[a] as usize + 1;
      let value = RawBytes::new(value.clone(), a + 1, c);
      map.insert(key, value);
    }

    Ok(Kv(map))
  }
}

impl Kv {
  pub fn get<'a>(&self, k: &'a str) -> Option<&str> {
    &self.0.get(k)
  }
}

impl<'a> KvBuilder<'a> {
  pub fn insert(&mut self, key: &'a str, value: &'a str) {
    self.0.push((key, value));
  }
}
