 use std::net::UdpSocket;

fn main() {
    println!("Logs from your program will appear here!");

     let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
     let mut buf = [0; 512];
    
     loop {
         match udp_socket.recv_from(&mut buf) {
             Ok((size, source)) => {
                 let _received_data = String::from_utf8_lossy(&buf[0..size]);
                 println!("Received {} bytes from {}", size, source);
                 let response = [];
                 udp_socket
                     .send_to(&response, source)
                     .expect("Failed to send response");
             }
             Err(e) => {
                 eprintln!("Error receiving data: {}", e);
                 break;


                 pub struct DNSserver {
                    id: u16,
                    qr: u16,
                    opcode: u8,
                    aa: u8,
                    tc: u8,
                    rd: u8,
                    ra: u8,
                    z: u8;
                    rcode: u8,
                    qdcount: u16,
                    ancount: u16,
                    nscount: u16,
                    arcount: u16,

                    let id_bytes = header.id.to_be_bytes();
                    buffer.extend_from_slice(&id_bytes);
                 }
             }
         }
     }
}
