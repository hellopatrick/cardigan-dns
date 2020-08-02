use super::{name::Name, record_type::RecordType};
use crate::Buffer;
use bytes::{Buf, BufMut};

#[derive(Debug, Clone)]
pub struct Question {
  name: Name,
  record_type: RecordType,
  class: u16,
}

impl Question {
  pub fn new(name: &str, record_type: RecordType) -> Self {
    Self {
      name: name.into(),
      record_type,
      class: 0x1,
    }
  }

  pub fn parse(buf: &mut Buffer) -> Self {
    let name = Name::parse(buf);
    let record_type = buf.get_u16().into();
    let class = buf.get_u16();

    Self {
      name,
      record_type,
      class,
    }
  }

  pub fn write(&self, buf: &mut Buffer) {
    self.name.write(buf);

    let record_type = self.record_type.into();

    buf.put_u16(record_type);
    buf.put_u16(self.class);
  }
}
