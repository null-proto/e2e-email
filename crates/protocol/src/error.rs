pub enum Error {
  InvalidFrame,
  InvalidKv,
  InvalidUtfString,
  InvalidString,
  ConnectionError
}

pub type Result<T> = core::result::Result<T, Error>;
