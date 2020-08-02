use bytes::{Buf, BufMut};
use std::{fmt::Debug, mem::MaybeUninit};

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

impl Debug for Buffer {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_fmt(format_args!("{:02x?}", &self.buf[..]))
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

impl BufMut for Buffer {
  fn remaining_mut(&self) -> usize {
    if self.pos > 512 {
      0
    } else {
      512 - self.pos
    }
  }

  unsafe fn advance_mut(&mut self, cnt: usize) {
    if cnt > self.remaining_mut() {
      panic!("unacceptable advance: {} > {}", cnt, self.remaining_mut())
    }

    self.pos += cnt;
  }

  fn bytes_mut(&mut self) -> &mut [MaybeUninit<u8>] {
    let m = self.buf[self.pos..].as_mut();
    let m = unsafe { std::mem::transmute::<&mut [u8], &mut [MaybeUninit<u8>]>(m) };
    m
  }
}
