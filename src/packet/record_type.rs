#[derive(Debug)]
pub enum RecordType {
  A,
  Unknown(u16),
}

impl From<u16> for RecordType {
  fn from(code: u16) -> Self {
    match code {
      1 => RecordType::A,
      _ => RecordType::Unknown(code),
    }
  }
}
