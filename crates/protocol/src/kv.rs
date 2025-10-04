use std::{collections::HashMap, sync::Arc};
use crate::bytes::Bytes;

pub struct Kv(HashMap<Bytes,Bytes>);

pub struct KvBuilder<'a>(&'a [(&'a str , &'a str)]);

impl TryFrom<&[u8]> for Kv {
  type Error = crate::error::Error;
  fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
    let mut map = HashMap::default();
    let value : Arc<[u8]> = Arc::from(value);

    let mut c = 0usize;
    let mut a = 0usize;

    while c < value.len() {
      a += value[c] as usize + 1;
      let key = Bytes::new(value.clone(), c+1, a);
      c += value[a] as usize + 1;
      let value = Bytes::new(value.clone(), a+1, c);
      map.insert(key,value);
    }

    Ok(Kv(map))
  }
}

