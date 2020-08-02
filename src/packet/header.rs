use crate::buffer::Buffer;
use bytes::{Buf, BufMut};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum ResponseCode {
  NoError = 0,
  FormatError = 1,
  ServerFailure = 2,
  NonexistentDomain = 3,
  NotImplemented = 4,
  Refused = 5,
}

impl From<u16> for ResponseCode {
  fn from(rc: u16) -> Self {
    match rc {
      1 => ResponseCode::FormatError,
      2 => ResponseCode::ServerFailure,
      3 => ResponseCode::NonexistentDomain,
      4 => ResponseCode::NotImplemented,
      5 => ResponseCode::Refused,
      0 | _ => ResponseCode::NoError,
    }
  }
}

#[derive(Debug, Copy, Clone)]
pub struct Header {
  pub id: u16,

  pub is_reply: bool,
  pub opcode: u8,

  pub is_authoritative: bool,
  pub is_truncated: bool,
  pub is_recusion_desired: bool,
  pub is_recusion_allowed: bool,
  pub z: u8,
  pub response_code: ResponseCode,

  pub question_count: u16,
  pub answer_count: u16,
  pub authority_resource_records_count: u16,
  pub additional_resource_records_count: u16,
}

impl Default for Header {
  fn default() -> Self {
    Self {
      id: 0,
      is_reply: false,
      opcode: 0,
      is_authoritative: false,
      is_truncated: false,
      is_recusion_desired: false,
      is_recusion_allowed: false,
      z: 0,
      response_code: ResponseCode::NoError,
      question_count: 0,
      answer_count: 0,
      authority_resource_records_count: 0,
      additional_resource_records_count: 0,
    }
  }
}

impl Header {
  pub fn parse(buf: &mut Buffer) -> Self {
    let mut res = Self::default();

    res.id = buf.get_u16();

    let mut header = buf.get_u16();

    let rcode = header & 0b1111;

    header >>= 4;
    let z = header & 0b111;

    header >>= 3;
    let ra = header & 0b1;

    header >>= 1;
    let rd = header & 0b1;

    header >>= 1;
    let tc = header & 0b1;

    header >>= 1;
    let aa = header & 0b1;

    header >>= 1;
    let op = header & 0b1111;

    header >>= 4;
    let qr = header & 0b1;

    res.is_reply = qr > 0;
    res.opcode = op as u8;
    res.is_authoritative = aa > 0;
    res.is_truncated = tc > 0;
    res.is_recusion_desired = rd > 0;
    res.is_recusion_allowed = ra > 0;
    res.z = z as u8;
    res.response_code = rcode.into();

    res.question_count = buf.get_u16();
    res.answer_count = buf.get_u16();
    res.authority_resource_records_count = buf.get_u16();
    res.additional_resource_records_count = buf.get_u16();

    res
  }

  pub fn write(&self, buf: &mut Buffer) {
    buf.put_u16(self.id);

    buf.put_u8(
      self.is_recusion_desired as u8
        | (self.is_truncated as u8) << 1
        | (self.is_authoritative as u8) << 2
        | self.opcode << 3
        | (self.is_reply as u8) << 7,
    );

    buf.put_u8(self.response_code as u8 | self.z << 4 | (self.is_recusion_allowed as u8) << 7);

    buf.put_u16(self.question_count);
    buf.put_u16(self.answer_count);
    buf.put_u16(self.authority_resource_records_count);
    buf.put_u16(self.additional_resource_records_count);
  }
}
