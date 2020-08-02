use bytes::Buf;
use cardigan::{
  packet::{Packet, Question},
  Buffer,
};
use std::net::UdpSocket;

fn make_packet(q: &Question) -> Packet {
  let mut p = Packet::default();

  p.header.id = 123;
  p.header.question_count = 1;
  p.header.is_recusion_desired = true;

  p.questions.push(q.clone());

  p
}

fn lookup(q: &Question) -> Packet {
  let server = ("8.8.8.8", 53);

  let socket = UdpSocket::bind(("0.0.0.0", 43210)).unwrap();

  let mut buffer = Buffer::default();

  let p = make_packet(q);
  p.write(&mut buffer);

  buffer.seek(0);

  // ...and send it off to the server using our socket:
  socket.send_to(buffer.bytes(), server).unwrap();

  // To prepare for receiving the response, we'll create a new `BytePacketBuffer`,
  // and ask the socket to write the response directly into our buffer.
  let mut src = [0; 512];

  socket.recv_from(&mut src).unwrap();
  let mut buf = Buffer::from_bytes(&src);

  Packet::parse(&mut buf)
}

fn handle_query(socket: &UdpSocket) {
  let mut bytes = [0; 512];
  let (_, addr) = socket.recv_from(&mut bytes).unwrap();
  let mut buf = Buffer::from_bytes(&bytes);

  let query = Packet::parse(&mut buf);
  let mut res = Packet::default();

  res.header.id = query.header.id;
  res.header.is_recusion_allowed = true;
  res.header.is_recusion_desired = true;
  res.header.is_reply = true;

  if let Some(q) = query.questions.first() {
    println!("question = {:?}", q);
    let p = lookup(q);

    res.header.response_code = p.header.response_code;

    res.header.question_count += 1;
    res.header.answer_count += p.answers.len() as u16;
    res.header.authority_resource_records_count += p.authority_resources.len() as u16;
    res.header.additional_resource_records_count += p.additional_resources.len() as u16;

    res.questions.push(q.clone());
    res.answers.extend(p.answers);
    res.authority_resources.extend(p.authority_resources);
    res.additional_resources.extend(p.additional_resources);
  }

  let mut b = Buffer::default();
  let len = res.write(&mut b);

  socket.send_to(&b.bytes()[..len], addr).expect("written");
}

fn main() {
  let socket = UdpSocket::bind(("0.0.0.0", 1123)).expect("socket");

  loop {
    handle_query(&socket);
  }
}
