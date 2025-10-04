pub enum Error {
  InvalidFrame,
  InvalidKv,
  InvalidUtfString,
  InvalidString,
}

pub type Result<T> = core::result::Result<T, Error>;
