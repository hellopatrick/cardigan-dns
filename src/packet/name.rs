use crate::Buffer;
use bytes::{Buf, BufMut};

const RECURSION_LIMIT: u8 = 16;

#[derive(Debug, Clone)]
pub struct Name(pub String);

impl Name {
  pub fn parse(buf: &mut Buffer) -> Self {
    Self::parse_aux(buf, 0)
  }

  fn parse_aux(buf: &mut Buffer, limit: u8) -> Self {
    if limit > RECURSION_LIMIT {
      panic!("TOO MUCH RECURSION: {}", limit);
    }

    let mut name = String::with_capacity(16);

    loop {
      let len = buf.get_u8();

      if len == 0 {
        break;
      }

      if len & 0xc0 == 0xc0 {
        // todo: avoid recursion cycle.

        let offset = ((len & 0x3f) as usize) << 8 | (buf.get_u8() as usize);

        let current = buf.pos();

        buf.seek(offset);

        let n = Name::parse_aux(buf, limit + 1);

        name.push_str(&n.0);

        buf.seek(current);

        break;
      } else {
        // straight forward
        let mut dst = vec![0; len as usize];
        buf.copy_to_slice(&mut dst);

        let s = std::str::from_utf8(&dst).expect("string");

        name.push_str(s);
        name.push('.');
      }
    }

    Self(name)
  }

  pub fn write(&self, buf: &mut Buffer) {
    let s = &self.0;
    let mut bytes = Vec::with_capacity(s.len());

    for part in s.split('.') {
      let len = part.len();

      if len > 0x3F {
        panic!("label too long")
      }

      bytes.push(len as u8);
      bytes.extend_from_slice(part.as_bytes());
    }

    buf.put_slice(&bytes);
  }

  pub fn len(&self) -> usize {
    let s = &self.0;
    s.split('.').map(|x| x.len() + 1).sum()
  }
}

impl From<&str> for Name {
  fn from(name: &str) -> Self {
    Self(name.into())
  }
}
