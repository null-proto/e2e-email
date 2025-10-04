
pub enum Error {
  InvalidFrame,
  InvalidKv,
}

pub type Result<T> = core::result::Result<T , Error>;
