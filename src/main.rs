use packed_struct::prelude::*;
use serde_derive::{Deserialize, Serialize};
use std::net::UdpSocket;

#[derive(PackedStruct, Debug)]
#[packed_struct(bit_numbering = "msb0")]
pub struct DNSHeader {
    #[packed_field(bits = "0..", endian = "msb")]
    id: u16,
    #[packed_field()]
    qr: bool,
    #[packed_field()]
    opcode: Integer<u8, packed_bits::Bits<4>>,
    #[packed_field()]
    aa: bool,
    #[packed_field()]
    tc: bool,
    #[packed_field()]
    rd: bool,
    #[packed_field()]
    ra: bool,
    #[packed_field()]
    _z: ReservedZeroes<packed_bits::Bits<3>>,
    #[packed_field()]
    rcode: Integer<u8, packed_bits::Bits<4>>,
    #[packed_field(endian = "msb")]
    qdcount: u16,
    #[packed_field(endian = "msb")]
    ancount: u16,
    #[packed_field(endian = "msb")]
    nscount: u16,
    #[packed_field(endian = "msb")]
    arcount: u16,
}

impl DNSHeader {
    fn to_bytes(&self) -> [u8; 12] {
        let mut buffer = [0; 12];
        buffer[0..2].copy_from_slice(&self.id.to_be_bytes());
        buffer[2] = (self.qr as u8) << 7
            | (u8::from(self.opcode) as u8) << 3
            | (self.aa as u8) << 2
            | (self.tc as u8) << 1
            | (self.rd as u8);
        buffer[3] = (self.ra as u8) << 7 | u8::from(self.rcode);
        buffer[4..6].copy_from_slice(&self.qdcount.to_be_bytes());
        buffer[6..8].copy_from_slice(&self.ancount.to_be_bytes());
        buffer[8..10].copy_from_slice(&self.nscount.to_be_bytes());
        buffer[10..12].copy_from_slice(&self.arcount.to_be_bytes());
        buffer
    }
}

impl Default for DNSHeader {
    fn default() -> Self {
        Self {
            id: 1234,
            qr: true,
            opcode: 0.into(),
            aa: false,
            tc: false,
            rd: false,
            ra: false,
            _z: Default::default(),
            rcode: 0.into(),
            qdcount: 1,
            ancount: 1,
            nscount: 0,
            arcount: 0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct DNSQuestion {
    domain_name: String,
    query_type: u16,
    query_class: u16,
}

impl DNSQuestion {
    fn to_bytes(&self) -> Vec<u8> {
        let mut buffer = vec![];
        self.domain_name.split('.').for_each(|split| {
            buffer.push(split.len() as u8);
            buffer.extend(split.as_bytes().iter().cloned());
        });
        buffer.push(0);
        buffer.extend(&self.query_type.to_be_bytes());
        buffer.extend(&self.query_class.to_be_bytes());
        buffer
    }
}

impl Default for DNSQuestion {
    fn default() -> Self {
        Self {
            domain_name: "codecrafters.io".to_string(),
            query_class: 1,
            query_type: 1,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct DNSAnswer {
    domain_name: Vec<u8>,
    query_class: u16,
    query_type: u16,
    ttl: u32,
    rdlength: u16,
    rdata: Vec<u8>,
}

impl DNSAnswer {
    fn to_bytes(&self) -> Vec<u8> {
        let mut buffer = vec![];
        buffer.extend(&self.domain_name);
        buffer.extend(&self.query_type.to_be_bytes());
        buffer.extend(&self.query_class.to_be_bytes());
        buffer.extend(&self.ttl.to_be_bytes());
        buffer.extend(&self.rdlength.to_be_bytes());
        buffer.extend(&self.rdata);
        buffer
    }
}

impl Default for DNSAnswer {
    fn default() -> Self {
        Self {
            domain_name: vec![
                0xc, 0x63, 0x6f, 0x64, 0x65, 0x63, 0x72, 0x61, 0x66, 0x74, 0x65, 0x72, 0x73, 0x2,
                0x69, 0x6f, 0x0,
            ],
            query_type: 1,
            query_class: 1,
            ttl: 60,
            rdlength: 4,
            rdata: vec![0x08, 0x08, 0x08, 0x08],
        }
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

                let received_header = DNSHeader::unpack_from_slice(&buf[0..=11]).unwrap();

                let mut response_header = DNSHeader::default();
                response_header.id = received_header.id;
                response_header.opcode = received_header.opcode;
                response_header.rd = received_header.rd;
                if received_header.opcode == 0.into() {
                    response_header.rcode = 0.into();
                } else {
                    response_header.rcode = 4.into();
                }

                let mut response = response_header.to_bytes().to_vec();
                response.extend_from_slice(&DNSQuestion::default().to_bytes());
                response.extend_from_slice(&DNSAnswer::default().to_bytes());

                // Send the response back to the client
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
