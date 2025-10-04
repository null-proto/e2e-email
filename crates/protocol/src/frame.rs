pub struct Frame {
  version: u8,
  size: u32,
  id: u8,
  data: Vec<u8>,
}

impl Frame {
  pub fn serialize<'a>(&'a self) -> Vec<u8> {
    let mut v = Vec::with_capacity(self.size as usize + 5);
    v.push(self.version);
    v.push(self.size as u8);
    v.push((self.size >> 8) as u8);
    v.push((self.size >> 16) as u8);
    v.push((self.size >> 24) as u8);
    v.push(self.id);
    v.extend_from_slice(&self.data);
    v
  }

  pub fn deserialize<'a>(data: &'a [u8]) -> Self {
    Self {
      version: data[0],
      size: ((data[1] as u32) << data[2]) << data[3],
      id: data[4],
      data: data[5..].to_vec(),
    }
  }
}
