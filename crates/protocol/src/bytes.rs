use std::fmt::Display;
use std::hash::Hash;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct RawBytes {
  data: Arc<[u8]>,
  start: usize, // Inclusicve
  end: usize,   // Exclusive
}

pub enum Bytes<'a> {
  Bytes(RawBytes),
  Str(&'a str),
}

unsafe impl Send for RawBytes {}
unsafe impl Sync for RawBytes {}

impl RawBytes {
  pub fn new(data: Arc<[u8]>, start: usize, end: usize) -> Self {
    Self {
      data: data.clone(),
      start: start,
      end: end,
    }
  }

  pub fn from<'a>(data: &'a [u8], start: usize, end: usize) -> Self {
    Self {
      data: Arc::from(data),
      start: start,
      end: end,
    }
  }
}

impl<'a> Bytes<'a> {
  pub fn from(data: &'a [u8], start: usize, end: usize) -> Self {
    Bytes::Bytes(RawBytes {
      data: Arc::from(data),
      start: start,
      end: end,
    })
  }

  pub fn from_atomic(data: Arc<[u8]>, start: usize, end: usize) -> Self {
    Bytes::Bytes(RawBytes {
      data: data.clone(),
      start: start,
      end: end,
    })
  }

  pub fn from_str(data: &'a str) -> Self {
    Bytes::Str(data)
  }
}

impl RawBytes {
  pub fn try_str<'a>(&'a self) -> crate::error::Result<&'a str> {
    str::from_utf8(&self.data[self.start..self.end])
      .map_err(|_| crate::error::Error::InvalidUtfString)
  }
}

impl<'a> Bytes<'a> {
  pub fn try_str(&'a self) -> crate::error::Result<&'a str> {
    match self {
      Bytes::Bytes(a) => a.try_str(),
      Bytes::Str(a) => Ok(a),
    }
  }
}

impl Hash for RawBytes {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    self.try_str().unwrap_or("\r").hash(state);
  }
}

impl Hash for Bytes<'_> {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    match self {
      Bytes::Bytes(a) => a.hash(state),
      Bytes::Str(a) => a.hash(state),
    }
  }
}

impl PartialEq for RawBytes {
  fn eq(&self, other: &Self) -> bool {
    match self.try_str() {
      Ok(v) => match other.try_str() {
        Ok(w) => v == w,
        Err(_) => false,
      },
      Err(_) => false,
    }
  }
}

impl PartialEq for Bytes<'_> {
  fn eq(&self, other: &Self) -> bool {
    match self {
      Bytes::Bytes(a) => match other {
        Bytes::Bytes(b) => a == b,
        Bytes::Str(b) => match a.try_str() {
          Ok(a) => a == *b,
          Err(_) => false,
        },
      },

      Bytes::Str(a) => match other {
        Bytes::Bytes(b) => match b.try_str() {
          Ok(b) => *a == b,
          Err(_) => false,
        },
        Bytes::Str(b) => a == b,
      },
    }
  }
}

impl Eq for RawBytes {}
impl Eq for Bytes<'_> {}

impl<'a> Into<Bytes<'a>> for RawBytes {
  fn into(self) -> Bytes<'a> {
    Bytes::Bytes(self)
  }
}

impl<'a> Into<Bytes<'a>> for &'a str {
  fn into(self) -> Bytes<'a> {
    Bytes::Str(self)
  }
}

impl Display for RawBytes {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.try_str().unwrap_or("---"))
  }
}

impl Display for Bytes<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.try_str().unwrap_or("---"))
  }
}
