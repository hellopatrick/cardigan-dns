use super::header::Header;
use super::question::Question;
use super::record::Record;
use crate::buffer::Buffer;

#[derive(Debug)]
pub struct Packet {
  header: Header,
  questions: Vec<Question>,
  answers: Vec<Record>,
}

impl Packet {
  pub fn parse(buf: &mut Buffer) -> Self {
    let header = Header::parse(buf);

    let qs = header.question_count;

    let questions = (0..qs).map(|_| Question::parse(buf)).collect();

    let ans = header.answer_count;

    let answers = (0..ans).map(|_| Record::parse(buf)).collect();

    Self {
      header,
      questions,
      answers,
    }
  }
}
