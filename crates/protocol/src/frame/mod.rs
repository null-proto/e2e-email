use std::io::Read;

use crate::{
  error::{Error, Result},
  kv::Kv,
};

pub mod builder;

//      A Frame is the minimum level of network transaction and it is only used in the networkstack.
//   First 96 bits in the frame is fixed length that tells information about remainings. Current
//   definition only suitable for TCP connection.
//
//
//   0                7                 15                23                 31
//   +------------------------------------------------------------------------+   0
//   |    version     |      ftype      |      flags      |        id         |
//   +------------------------------------------------------------------------+   32
//   |                id                |        frame size (in octans)       |
//   +------------------------------------------------------------------------+   64
//   |                               padding                                  |
//   +------------------------------------------------------------------------+   96
//   |                               payload                                  |
//   +------------------------------------------------------------------------+   ...
//
//   # version (8-bits)
//   Version is always set to 0x01.
//
//   # Frame Type (8-bits)
//   ```c
//   enum FRAME_TYPE {
//     TEXT = 1,
//     KEY_VALUE = 2,
//     RAW = 3,
//     PING = 4,
//   }
//   ```
//
//   Frame type tells it payload format, helps to reconstruce its header/body. The Frame Type
//   MUST NOT BE 0x00. If frame type is 0x00 it means connection error.
//
//   >  TEXT      :  UTF-8 String
//   >  KEY_VALUE :  http2 like kv
//   >  RAW       :  binary data
//   >  PING      :  Ping is a special type that assumed not contains payload, frame size will
//   >               discarded.
//
//
//   # Flags (8-bits)
//   first bit is set if the frame is the last part of data.
//
//   # Id (24-bits)
//   Frame identifier, used to reconstruct multiplexed frame just like http2. Zero(0x00) is
//   special id that means data is not multiplexed.
//
//   # Frame Size ( 16-bits)
//   Frame size is a 2 octant unsigned integer of the payload size in OCTANS(or Byte). It MAYBE
//   0x00 in certain frame types.
//
//   # Padding (32-bit)
//   32bit padding after frame size, that MUSTBE 0x00.
//
pub struct Frame<'a> {
  pub(crate) version: u8,
  pub(crate) flags: FrameFlags,
  pub(crate) id: (u8, u8, u8),
  pub(crate) size: u16,
  pub(crate) data: FrameType<'a>,
}

pub struct FrameFlags {
  pub(crate) last_frame: bool,
}

pub enum FrameType<'a> {
  Kv(Kv<'a>),
  Raw(Box<[u8]>),
  Text(String),
  Ping,
}

impl<'a> Frame<'a> {
  pub fn new<T>(io: &mut T) -> Result<Self>
  where
    T: Read,
  {
    let mut buf = [0u8; 12];
    _ = io.read(&mut buf);

    let version = if buf[0] == 0x01 {
      Ok(0x01)
    } else {
      Err(Error::FrameErrorUnsupportedVersion)
    }?;

    let ftype = buf[1];
    let flags = FrameFlags {
      last_frame: (buf[2] >> 7) == 1,
    };

    let id: (u8, u8, u8) = (buf[3], buf[4], buf[5]);
    let size: u16 = (buf[6] as u16) << 8 | (buf[7] as u16);

    let data = match ftype {
      0x00 => Err(Error::ConnectionError),
      0x01 => {
        // text
        let mut data = vec![0u8; size as usize];
        io.read(&mut data).map_err(|_| Error::ConnectionError)?;
        Ok(FrameType::Text(
          String::from_utf8(data).map_err(|_| Error::InvalidString)?,
        ))
      }
      0x02 => {
        // kv
        let mut data = vec![0u8; size as usize].into_boxed_slice();
        io.read(&mut data).map_err(|_| Error::ConnectionError)?;
        let kv = Kv::try_from(data.as_ref())?;
        Ok(FrameType::Kv(kv))
      }
      0x03 => {
        // raw
        let mut data = vec![0u8; size as usize].into_boxed_slice();
        io.read(&mut data).map_err(|_| Error::ConnectionError)?;
        Ok(FrameType::Raw(data))
      }

      0x04 => Ok(FrameType::Ping),
      _ => Err(Error::InvalidFrame),
    }?;

    Ok(Self {
      version,
      flags,
      id,
      size,
      data,
    })
  }
}
