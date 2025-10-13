use crate::kv::Kv;
use crate::kv::KvBuilder;
use crate::serde::Serde;

pub struct FrameBuilder {
  pub(crate) id: (u8, u8, u8),
  pub(crate) ftype: u8,
  pub(crate) flags: FrameBuilderFlags,
  pub(crate) data: Option<Box<[u8]>>,
}

pub struct FrameBuilderFlags {
  pub(crate) last_frame: bool,
}

impl Serde for FrameBuilder {
  fn serialize(&self) -> Vec<u8> {
    let flags = if self.flags.last_frame {
      0b1000_0000u8
    } else {
      0x00u8
    };
    let mut data = Vec::with_capacity(12usize + self.data.as_ref().map(|i| i.len()).unwrap_or(0));

    if let Some(d) = &self.data {
      let size = d.len() as u16;

      data.extend_from_slice(&[
        0x01,
        self.ftype,
        flags,
        self.id.0,
        self.id.1,
        self.id.2,
        size as u8,
        (size >> 8) as u8,
        0x00,
        0x00,
        0x00,
        0x00,
      ]);
      data.extend_from_slice(d);
    } else {
      data.extend_from_slice(&[
        0x01, self.ftype, flags, self.id.0, self.id.1, self.id.2, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00,
      ]);
    }

    data
  }
}

impl FrameBuilder {
  pub fn ping() -> Self {
    Self {
      id: (0, 0, 0),
      ftype: 0x04,
      flags: FrameBuilderFlags { last_frame: false },
      data: None,
    }
  }

  pub fn fin(id: (u8, u8, u8)) -> Self {
    Self {
      id,
      ftype: 0x05,
      flags: FrameBuilderFlags { last_frame: false },
      data: None,
    }
  }

  pub fn builder() -> FrameBuilder {
    Self {
      id: (0, 0, 0),
      ftype: 0,
      flags: FrameBuilderFlags { last_frame: true },
      data: None,
    }
  }

  pub fn flag_last(mut self, f: bool) -> Self {
    self.flags.last_frame = f;
    self
  }

  pub fn id(mut self, id: (u8, u8, u8)) -> Self {
    self.id = id;
    self
  }

  pub fn attach_raw_data(mut self, data: Box<[u8]>) -> Self {
    self.ftype = 0x03;
    self.data = Some(data);
    self
  }

  pub fn attach_kv<'a>(mut self, kv: Kv<'a>) -> Self {
    self.ftype = 0x02;
    self.data = Some(kv.serialize().into());
    self
  }

  pub fn attach_kvbuilder<'a>(mut self, kv: KvBuilder<'a>) -> Self {
    self.ftype = 0x02;
    self.data = Some(kv.serialize().into());
    self
  }

  pub fn attach_text<'a>(mut self, s: &'a str) -> Self {
    self.ftype = 0x01;
    self.data = Some(s.as_bytes().into());
    self
  }
}
