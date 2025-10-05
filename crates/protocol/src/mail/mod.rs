use std::sync::Arc;

use crate::bytes::RawBytes;


pub(crate) struct Header {
  from : RawBytes,
  to : RawBytes,
  subject : RawBytes,
  mime : RawBytes,
}

pub(crate) struct Body(Arc<[u8]>);


pub struct Mail {
  head : Header,
  body : Body
}
