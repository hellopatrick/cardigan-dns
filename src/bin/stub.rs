use bytes::Buf;
use cardigan::packet::{Packet, Question, RecordType};
use cardigan::Buffer;
use std::net::UdpSocket;

fn make_packet() -> Packet {
  let mut p = Packet::default();

  p.header.id = 123;
  p.header.question_count = 1;
  p.header.is_recusion_desired = true;

  p.questions
    .push(Question::new("yahoo.com.", RecordType::AAAA));

  p
}

fn main() {
  let p = make_packet();

  let server = ("8.8.8.8", 53);

  let socket = UdpSocket::bind(("0.0.0.0", 43210)).unwrap();

  let mut buffer = Buffer::default();
  p.write(&mut buffer);

  buffer.seek(0);

  // ...and send it off to the server using our socket:
  socket.send_to(buffer.bytes(), server).unwrap();

  // To prepare for receiving the response, we'll create a new `BytePacketBuffer`,
  // and ask the socket to write the response directly into our buffer.
  let mut src = [0; 512];

  socket.recv_from(&mut src).unwrap();
  let mut buf = Buffer::from_bytes(&src);

  let p = Packet::parse(&mut buf);

  println!("packet = {:#?}", p)
}
