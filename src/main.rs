use cardigan::packet::Packet;

fn main() {
  let bytes = include_bytes!("../data/cnn_query_packet.txt");
  let mut buf = cardigan::Buffer::from_bytes(bytes);
  let packet = Packet::parse(&mut buf);

  println!("buffer- = {:?}", buf);
  println!("packet = {:?}", packet);

  let mut buf = cardigan::Buffer::default();
  packet.write(&mut buf);

  println!("buffer+ = {:?}", buf);

  let bytes = include_bytes!("../data/cnn_response_packet.txt");
  let mut buf = cardigan::Buffer::from_bytes(bytes);
  let packet = Packet::parse(&mut buf);

  println!("buffer- = {:?}", buf);
  println!("packet = {:?}", packet);

  let mut buf = cardigan::Buffer::default();
  packet.write(&mut buf);

  println!("buffer+ = {:?}", buf);
}
