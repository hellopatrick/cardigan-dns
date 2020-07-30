use super::{name::Name, record_type::RecordType};
use crate::Buffer;
use bytes::Buf;

#[derive(Debug)]
pub struct Question {
  name: Name,
  record_type: RecordType,
  class: u16,
}

impl Question {
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
}
