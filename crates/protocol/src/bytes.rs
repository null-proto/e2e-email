use std::{hash::Hash, str::Utf8Error, sync::Arc};


#[derive(Debug , Clone )]
pub struct Bytes {
  data : Arc<[u8]>,
  start : usize, // Inclusicve
  end : usize,   // Exclusive
}

unsafe impl Send for Bytes {}
unsafe impl Sync for Bytes {}

impl Bytes {
  pub fn new(data :Arc<[u8]> , start : usize , end : usize) -> Self {
    Self { data: data.clone(), start: start, end: end }
  }


  pub fn from_vec(data : Vec<u8> , start : usize , end : usize) -> Self {
    Self { data: Arc::from(data), start: start, end: end }
  }
}


impl Bytes {
  pub fn try_str<'a >(&'a self) -> Result<&'a str, Utf8Error> {
    str::from_utf8(&self.data[self.start .. self.end])
  }
}

impl Hash for Bytes {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    self.try_str().unwrap_or("\r").hash(state);
  }
}

impl PartialEq for Bytes {
  fn eq(&self, other: &Self) -> bool {
    match self.try_str() {
      Ok(v) =>{
        match other.try_str() {
          Ok(w) => {
            v == w
          }

          Err(_) => false
        }
      }

      Err(_) => false
    }
  }
}

impl Eq for Bytes {}
