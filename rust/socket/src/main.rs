use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;
use std::net::UdpSocket;

fn send_msg(sock: UdpSocket, msg: String,) -> anyhow::Result<(),> {
	let (mut sent_len, msg_len,) = (0, msg.len(),);
	while sent_len < msg_len {
		match sock.send(msg[sent_len..].as_bytes(),) {
			Ok(i,) if i != 0 => sent_len += i,
			_ => anyhow::bail!("send failed"),
		}
	}
	Ok((),)
}

fn recv_msg(sock: UdpSocket,) {
	let mut buf = [0; 1024];
	while let Ok(i,) = sock.recv(&mut buf,) {
		if i == 0 {
			break;
		}
		println!("|> {}", std::str::from_utf8(&buf[..i]).unwrap());
	}
}

fn hndl_cnct(mut stream: TcpStream,) {
	let buf_reader = BufReader::new(&mut stream,);
	let http_request: Vec<_,> = buf_reader
		.lines()
		.map(|rslt| rslt.unwrap(),)
		.take_while(|line| !line.is_empty(),)
		.collect();
	println!("{http_request:#?}");
	let response = "HTTP/1.1 200 OK\r\n\r\n";
	stream.write_all(response.as_bytes(),).unwrap();
}

fn main() -> anyhow::Result<(),> {
	let listener = TcpListener::bind("127.0.0.1:80",)?;
	let sock = UdpSocket::bind("127.0.0.1:80",)?;
	let listen = true;
	let cnct = false;
	if listen == true {
		recv_msg(sock,);
	} else if cnct == true {
		for stream in listener.incoming() {
			hndl_cnct(stream?,);
		}
	} else {
		send_msg(sock, "hello from Rust \u{e7a8}".to_string(),)?;
	}

	Ok((),)
}
