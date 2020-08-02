use super::name::Name;
use crate::Buffer;
use bytes::{Buf, BufMut};
use std::net::{Ipv4Addr, Ipv6Addr};

#[derive(Debug, Clone)]
pub enum Record {
  A {
    name: Name,
    class: u16,
    ttl: u32,
    addr: Ipv4Addr,
  },
  NS {
    name: Name,
    class: u16,
    ttl: u32,
    host: Name,
  },
  CNAME {
    name: Name,
    class: u16,
    ttl: u32,
    host: Name,
  },
  MX {
    name: Name,
    class: u16,
    ttl: u32,
    priority: u16,
    host: Name,
  },
  AAAA {
    name: Name,
    class: u16,
    ttl: u32,
    addr: Ipv6Addr,
  },
  Unimplemented {
    name: Name,
    kind: u16,
    class: u16,
    ttl: u32,
    data: Vec<u8>,
  },
}

impl Record {
  pub fn parse(buf: &mut Buffer) -> Self {
    let name = Name::parse(buf);
    let kind = buf.get_u16();
    let class = buf.get_u16();
    let ttl = buf.get_u32();

    match kind {
      1 => Self::parse_a(name, class, ttl, buf),
      2 => Self::parse_ns(name, class, ttl, buf),
      5 => Self::parse_canonical(name, class, ttl, buf),
      15 => Self::parse_mx(name, class, ttl, buf),
      28 => Self::parse_aaaa(name, class, ttl, buf),
      _ => Self::parse_unimplemented(name, kind, class, ttl, buf),
    }
  }

  fn parse_a(name: Name, class: u16, ttl: u32, buf: &mut Buffer) -> Self {
    let _ = buf.get_u16();

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

  fn parse_ns(name: Name, class: u16, ttl: u32, buf: &mut Buffer) -> Self {
    let _ = buf.get_u16();
    let host = Name::parse(buf);

    Record::NS {
      name,
      class,
      ttl,
      host,
    }
  }

  fn parse_canonical(name: Name, class: u16, ttl: u32, buf: &mut Buffer) -> Self {
    let _ = buf.get_u16();
    let host = Name::parse(buf);

    Record::CNAME {
      name,
      class,
      ttl,
      host,
    }
  }

  fn parse_mx(name: Name, class: u16, ttl: u32, buf: &mut Buffer) -> Self {
    let _ = buf.get_u16();
    let priority = buf.get_u16();
    let host = Name::parse(buf);

    Record::MX {
      name,
      class,
      ttl,
      priority,
      host,
    }
  }

  fn parse_aaaa(name: Name, class: u16, ttl: u32, buf: &mut Buffer) -> Self {
    let _ = buf.get_u16();

    let addr = Ipv6Addr::new(
      buf.get_u16(),
      buf.get_u16(),
      buf.get_u16(),
      buf.get_u16(),
      buf.get_u16(),
      buf.get_u16(),
      buf.get_u16(),
      buf.get_u16(),
    );

    Record::AAAA {
      name,
      class,
      ttl,
      addr,
    }
  }

  fn parse_unimplemented(name: Name, kind: u16, class: u16, ttl: u32, buf: &mut Buffer) -> Self {
    let data_len = buf.get_u16() as usize;

    let mut data = vec![0u8; data_len];

    buf.copy_to_slice(&mut data);

    Record::Unimplemented {
      name,
      kind,
      class,
      ttl,
      data,
    }
  }

  pub fn write(&self, buf: &mut Buffer) {
    match self {
      Record::A {
        name,
        class,
        ttl,
        addr,
      } => {
        name.write(buf);
        buf.put_u16(0x1);
        buf.put_u16(*class);
        buf.put_u32(*ttl);
        buf.put_u16(0x4);
        let octets = addr.octets();
        buf.put_slice(&octets);
      }
      Record::NS {
        name,
        class,
        ttl,
        host,
      } => {
        name.write(buf);
        buf.put_u16(2);
        buf.put_u16(*class);
        buf.put_u32(*ttl);
        buf.put_u16(host.len() as u16);
        host.write(buf);
      }
      Record::CNAME {
        name,
        class,
        ttl,
        host,
      } => {
        name.write(buf);
        buf.put_u16(5);
        buf.put_u16(*class);
        buf.put_u32(*ttl);
        buf.put_u16(host.len() as u16);
        host.write(buf);
      }
      Record::MX {
        name,
        class,
        ttl,
        priority,
        host,
      } => {
        name.write(buf);
        buf.put_u16(15);
        buf.put_u16(*class);
        buf.put_u32(*ttl);
        buf.put_u16((1 + host.len()) as u16);
        buf.put_u16(*priority);
        host.write(buf);
      }
      Record::AAAA {
        name,
        class,
        ttl,
        addr,
      } => {
        name.write(buf);
        buf.put_u16(28);
        buf.put_u16(*class);
        buf.put_u32(*ttl);
        buf.put_u16(0x8);
        let octets = addr.octets();
        buf.put_slice(&octets);
      }
      Record::Unimplemented {
        name,
        kind,
        class,
        ttl,
        data,
      } => {
        name.write(buf);
        buf.put_u16(*kind);
        buf.put_u16(*class);
        buf.put_u32(*ttl);
        buf.put_u16(data.len() as u16);
        buf.put_slice(&data);
      }
    }
  }
}
