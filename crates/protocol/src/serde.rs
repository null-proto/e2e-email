use crate::error::Result;

pub trait Serde {
  fn serialize(&self) -> Vec<u8>;
}

pub trait Deser<'a> {
  type Type;
  fn deserialize(data: &'a [u8]) -> Result<Self::Type>;
}
