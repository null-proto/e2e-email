pub enum Error {
  InvalidFrame,
  InvalidKv,
  InvalidUtfString,
  InvalidString,
  ConnectionError,
  FrameErrorUnsupportedVersion
}

pub type Result<T> = core::result::Result<T, Error>;
