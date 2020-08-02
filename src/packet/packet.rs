use super::header::Header;
use super::question::Question;
use super::record::Record;
use crate::buffer::Buffer;

#[derive(Debug, Default, Clone)]
pub struct Packet {
  pub header: Header,
  pub questions: Vec<Question>,
  pub answers: Vec<Record>,
  pub authority_resources: Vec<Record>,
  pub additional_resources: Vec<Record>,
}

impl Packet {
  pub fn parse(buf: &mut Buffer) -> Self {
    let header = Header::parse(buf);

    let qs = header.question_count;

    let questions = (0..qs).map(|_| Question::parse(buf)).collect();

    let ans = header.answer_count;

    let answers = (0..ans).map(|_| Record::parse(buf)).collect();

    let auths = header.authority_resource_records_count;

    let authority_resources = (0..auths).map(|_| Record::parse(buf)).collect();

    let adds = header.additional_resource_records_count;

    let additional_resources = (0..adds).map(|_| Record::parse(buf)).collect();

    Self {
      header,
      questions,
      answers,
      authority_resources,
      additional_resources,
    }
  }

  pub fn write(&self, buf: &mut Buffer) -> usize {
    let start = buf.pos();

    self.header.write(buf);
    self.questions.iter().for_each(|q| q.write(buf));
    self.answers.iter().for_each(|q| q.write(buf));
    self.authority_resources.iter().for_each(|q| q.write(buf));
    self.additional_resources.iter().for_each(|q| q.write(buf));

    let end = buf.pos();

    buf.seek(start);

    end
  }
}
