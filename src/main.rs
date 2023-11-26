use bincode::{deserialize, serialize};
use serde_derive::{Deserialize, Serialize};
use std::io::Write;
use std::net::UdpSocket;

#[derive(Debug, Serialize, Deserialize)]
pub struct DNSHeader {
    id: u16,
    qr: u16,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct DNSQuestion {
    // Placeholder for DNS question structure
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DNSMessage {
    header: DNSHeader,
    question: DNSQuestion,
}

impl DNSMessage {
    fn write<W: Write>(&self, writer: &mut W) -> Result<(), Box<dyn std::error::Error>> {
        let buf = serialize(self)?;
        writer.write_all(&buf)?;
        Ok(())
    }
}

impl DNSMessage {
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
        serialize(self).expect("Failed to serialize DNS-Header")
    }

    fn from_bytes(data: &[u8]) -> DNSHeader {
        deserialize(data).expect("Failed to deserialize DNS-Header")
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
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                break;
            }
        }
    }
}
