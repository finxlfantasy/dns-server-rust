use bincode::{deserialize, serialize};
use serde::{Deserialize, Serialize};
use serde_derive::{Deserialize, Serialize};
use std::io::{self, Read, Write};
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

impl DNSHeader {
    fn from_reader<R: Read>(reader: &mut R) -> io::Result<Self> {
        let mut buf = vec![0; 12];
        reader.read_exact(&mut buf)?;
        let header = deserialize(&buf).unwrap();
        Ok(header)
    }

    fn write<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        let buf = serialize(self).unwrap();
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
                let _received_data = String::from_utf8_lossy(&buf[0..size]);
                println!("Received {} bytes from {}", size, source);
                let response = [];
                udp_socket
                    .send_to(&response, source)
                    .expect("Failed to send response");

                let response_header = DNSHeader {
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
                };

                let mut response_buffer = Vec::new();
                response_header.write(&mut response_buffer);

                udp_socket.send_to(&response_buffer, source);
                response_buffer = Vec::new();
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                break;
            }
        }
    }
}
