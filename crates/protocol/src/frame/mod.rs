use crate::kv::Kv;

pub mod reader;

pub enum FrameData<'a> {
  KvStore(Kv<'a>),
  Text(String),
  Raw(u8),
}
