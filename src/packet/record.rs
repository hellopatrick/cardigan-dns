use super::name::Name;
use crate::Buffer;
use bytes::Buf;
use std::net::Ipv4Addr;

#[derive(Debug)]
pub enum Record {
  A {
    name: Name,
    class: u16,
    ttl: u32,
    addr: Ipv4Addr,
  },
  CNAME {
    name: Name,
    class: u16,
    ttl: u32,
    canonical_name: Name,
  },
  Unimplemented {
    name: Name,
    kind: u16,
    class: u16,
    ttl: u32,
    data_len: u16,
  },
}

impl Record {
  pub fn parse(buf: &mut Buffer) -> Self {
    let name = Name::parse(buf);
    let kind = buf.get_u16();
    let class = buf.get_u16();
    let ttl = buf.get_u32();

    match kind {
      0x1 => Self::parse_a(name, class, ttl, buf),
      0x5 => Self::parse_canonical(name, class, ttl, buf),
      _ => Self::parse_unimplemented(name, kind, class, ttl, buf),
    }
  }

  fn parse_a(name: Name, class: u16, ttl: u32, buf: &mut Buffer) -> Self {
    let data_len = buf.get_u16();

    assert_eq!(data_len, 4);

    let a = buf.get_u8();
    let b = buf.get_u8();
    let c = buf.get_u8();
    let d = buf.get_u8();

    let addr = Ipv4Addr::new(a, b, c, d);

    Record::A {
      name,
      class,
      ttl,
      addr,
    }
  }

  fn parse_canonical(name: Name, class: u16, ttl: u32, buf: &mut Buffer) -> Self {
    let _ = buf.get_u16();
    let canonical_name = Name::parse(buf);

    Record::CNAME {
      name,
      class,
      ttl,
      canonical_name,
    }
  }

  fn parse_unimplemented(name: Name, kind: u16, class: u16, ttl: u32, buf: &mut Buffer) -> Self {
    let data_len = buf.get_u16();

    for _ in 0..data_len {
      buf.get_u8();
    }

    Record::Unimplemented {
      name,
      kind,
      class,
      ttl,
      data_len,
    }
  }
}
