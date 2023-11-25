use bincode::serialize;
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

const BUFFER_SIZE: usize = 512;
fn main() {
    println!("Logs from your program will appear here!");

    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; BUFFER_SIZE];

    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                println!("Received {} bytes from {}", size, source);

                let response_message = DNSMessage {
                    header: DNSHeader {
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
                    },
                    question: DNSQuestion {
                        // Fill in question details here
                    },
                };

                let mut response_buffer = Vec::new();
                if let Err(e) = response_message.write(&mut response_buffer) {
                    eprintln!("Error writing message: {}", e);
                    continue;
                }

                let _ = udp_socket.send_to(&response_buffer, source);
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                break;
            }
        }
    }
}
