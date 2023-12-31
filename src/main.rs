mod server;

use std::net::UdpSocket;
fn main() {
    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; 512];
    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                println!("Received {} bytes from {}", size, source);
                match server::DNSQuery::from_bytes(&buf[0..size]) {
                    Ok(query) => {
                        //println!("Query: {:?}", query);
                        let response = server::DNSPacket::for_request(query);
                        //println!("Response: {:?}", response);
                        udp_socket
                            .send_to(&response.to_bytes(), source)
                            .expect("Failed to send response");
                    }
                    Err(e) => {
                        eprintln!("Error parsing query: {}", e);
                        continue;
                    }
                }
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                break;
            }
        }
    }
}
