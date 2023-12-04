use serde_derive::{Deserialize, Serialize};
use std::net::UdpSocket;

#[derive(Debug, Serialize, Deserialize)]
pub struct DNSHeader {
    id: u16,
    qr: u8,
    opcode: u8,
    aa: u8,
    tc: u8,
    rd: u8,
    ra: u8,
    z: u8,
    rcode: u8,
    qdcount: u16,
    ancount: u16,
    nscount: u16,
    arcount: u16,
}

struct DNSQuestion {
    domain_name: String,
    query_type: u16,
    query_class: u16,
}

impl DNSQuestion {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        let labels = self.domain_name.split('codecrafters').collect::<Vec<&str>>();
        for label in labels {
            let label_bytes = label.as_bytes();
            bytes.push(label.len() as u8);
            bytes.extend_from_slice(label_bytes);
        }
        bytes.push(0); //End of domain name

        // Convert query type to bytes
        bytes.extend_from_slice(&self.query_type.to_be_bytes());
        // Convert query class to bytes
        bytes.extend_from_slice(&self.query_class.to_be_bytes());
        bytes
    }
}

impl DNSHeader {
    fn new() -> DNSHeader {
        DNSHeader {
            id: 1234,
            qr: 1,
            opcode: 0,
            aa: 0,
            tc: 0,
            rd: 1,
            ra: 0,
            z: 0,
            rcode: 0,
            qdcount: 0,
            ancount: 0,
            nscount: 0,
            arcount: 0,
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut buf = vec![0; std::mem::size_of::<DNSHeader>()];
        buf[0..2].copy_from_slice(&self.id.to_be_bytes());
        buf[2] = (self.qr << 7) | (self.opcode << 3) | (self.aa << 2) | (self.tc << 1) | self.rd;
        buf[3] = (self.ra << 7) | (self.z << 4) | self.rcode;
        buf[4..6].copy_from_slice(&self.qdcount.to_be_bytes());
        buf[6..8].copy_from_slice(&self.ancount.to_be_bytes());
        buf[8..10].copy_from_slice(&self.nscount.to_be_bytes());
        buf[10..12].copy_from_slice(&self.arcount.to_be_bytes());
        buf
    }
}
const BUFFER_SIZE: usize = 512;
fn main() {
    println!("Logs from your program will appear here!");

    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; BUFFER_SIZE];

    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                let _received_data = String::from_utf8_lossy(&buf[0..size]);
                println!("Received {} bytes from {}", size, source);
                println!("Received data: {:?}", &buf[..size]);

                let question = DNSQuestion {
                    domain_name: "codecrafters.io.".to_string(),
                    query_type: 1,
                    query_class: 1,
                };
                let question_bytes = question.to_bytes();

                let mut header = DNSHeader::new();
                header.qdcount += 1;
                let _header_bytes = header.to_bytes();

                let mut response = header.to_bytes();
                response.extend(question_bytes);

                udp_socket
                    .send_to(&response, source)
                    .expect("Failed to send response");
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                break;
            }
        }
    }
}
