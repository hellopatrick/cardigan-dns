#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum RecordType {
  A,
  NS,
  CNAME,
  MX,
  AAAA,
  ANY,
  Unknown(u16),
}

impl From<u16> for RecordType {
  fn from(code: u16) -> Self {
    match code {
      1 => RecordType::A,
      2 => RecordType::NS,
      5 => RecordType::CNAME,
      15 => RecordType::MX,
      28 => RecordType::AAAA,
      255 => RecordType::ANY,
      _ => RecordType::Unknown(code),
    }
  }
}

impl From<RecordType> for u16 {
  fn from(record_type: RecordType) -> Self {
    match record_type {
      RecordType::Unknown(code) => code,
      RecordType::A => 1,
      RecordType::NS => 2,
      RecordType::CNAME => 5,
      RecordType::MX => 15,
      RecordType::AAAA => 28,
      RecordType::ANY => 255,
    }
  }
}
