use crate::serde::Serde;

pub struct FrameBuilder {
  pub(crate) id: (u8, u8, u8),
  pub(crate) size : u16,
  pub(crate) ftype : u8,
  pub(crate) flags: FrameBuilderFlags,
  pub(crate) data: Box<[u8]>,
}

pub struct FrameBuilderFlags {
  pub(crate) last_frame: bool,
}

impl Serde for FrameBuilder {
  fn serialize(&self) -> Vec<u8> {
    let flags = if self.flags.last_frame { 0b1000_0000u8 } else { 0x00u8 };

    let mut data = Vec::with_capacity( 12usize + self.size as usize );

    data.extend_from_slice( &[
      0x01, self.ftype , flags , self.id.0 ,
      self.id.1 , self.id.2 , self.size as u8 , (self.size >> 8) as u8,
      0x00,0x00,0x00,0x00,
    ]);
    data.extend_from_slice(&self.data);

    data
  }
}
