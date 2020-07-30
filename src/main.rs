use cardigan::packet::Packet;

fn main() {
  let b = include_bytes!("../data/cnn_query_packet.txt");
  let mut b = cardigan::Buffer::from_bytes(b);

  let header = Packet::parse(&mut b);

  println!("query = {:#?}", header);

  let b = include_bytes!("../data/cnn_response_packet.txt");
  let mut b = cardigan::Buffer::from_bytes(b);

  let header = Packet::parse(&mut b);

  println!("resp = {:#?}", header);
}
