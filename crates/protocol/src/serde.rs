use crate::error::Result;


pub trait Serde {
  fn serialize(&self) -> Vec<u8>;
}

pub trait Deser {
  type Type;
  fn deserialize() -> Result<Self::Type>;
}
