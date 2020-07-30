use bytes::Buf;

pub struct Buffer {
  buf: [u8; 512],
  pos: usize,
}

impl Default for Buffer {
  fn default() -> Self {
    Self {
      buf: [0; 512],
      pos: 0,
    }
  }
}

impl Buffer {
  pub fn from_bytes(bytes: &[u8]) -> Self {
    let mut buffer = Buffer::default();

    buffer.buf[..bytes.len()].copy_from_slice(bytes);

    buffer
  }

  pub fn pos(&mut self) -> usize {
    self.pos
  }

  pub fn seek(&mut self, pos: usize) {
    self.pos = pos;
  }
}

impl Buf for Buffer {
  fn remaining(&self) -> usize {
    if self.pos > 512 {
      0
    } else {
      512 - self.pos
    }
  }

  fn bytes(&self) -> &[u8] {
    &self.buf[self.pos..]
  }

  fn advance(&mut self, cnt: usize) {
    self.pos += cnt;
  }
}
