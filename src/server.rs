use std::net::Ipv4Addr;

const HEADER_LENGTH: usize = 12;

#[derive(Debug, Clone)]
pub struct ServerError;
impl std::fmt::Display for ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "ServerError")
    }
}

#[derive(Debug)]
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

impl DNSHeader {
    pub fn new(id: u16, _response: bool) -> DNSHeader {
        DNSHeader {
            id,
            qr: 1,
            opcode: 0,
            aa: 0,
            tc: 0,
            rd: 0,
            ra: 0,
            z: 0,
            rcode: 0,
            qdcount: 0,
            ancount: 0,
            nscount: 0,
            arcount: 0,
        }
    }

    pub fn to_bytes(&self) -> [u8; HEADER_LENGTH] {
        let mut bytes = [0; HEADER_LENGTH];
        bytes[0..2].copy_from_slice(&self.id.to_be_bytes());
        bytes[2] = (self.qr << 7) | (self.opcode << 3) | (self.aa << 2) | (self.tc << 1) | self.rd;
        bytes[3] = (self.ra << 7) | (self.z << 4) | self.rcode;
        bytes[4..6].copy_from_slice(&self.qdcount.to_be_bytes());
        bytes[6..8].copy_from_slice(&self.ancount.to_be_bytes());
        bytes[8..10].copy_from_slice(&self.nscount.to_be_bytes());
        bytes[10..12].copy_from_slice(&self.arcount.to_be_bytes());
        bytes
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<DNSHeader, ServerError> {
        if bytes.len() < HEADER_LENGTH {
            Err(ServerError)
        } else {
            let bytes = &bytes[0..HEADER_LENGTH];
            Ok(DNSHeader {
                id: u16::from_be_bytes([bytes[0], bytes[1]]),
                qr: bytes[2] >> 7,
                opcode: (bytes[2] >> 3) & 0b1111,
                aa: bytes[2] & 0b100,
                tc: bytes[2] & 0b10,
                rd: bytes[2] & 0b1,
                ra: bytes[3] >> 7,
                z: (bytes[3] >> 4) & 0b111,
                rcode: bytes[3] & 0b1111,
                qdcount: u16::from_be_bytes([bytes[4], bytes[5]]),
                ancount: u16::from_be_bytes([bytes[6], bytes[7]]),
                nscount: u16::from_be_bytes([bytes[8], bytes[9]]),
                arcount: u16::from_be_bytes([bytes[10], bytes[11]]),
            })
        }
    }
}

#[derive(Debug)]
pub struct DNSQuestion {
    domain_name: Vec<String>,
    query_type: u16,
    query_class: u16,
}

impl DNSQuestion {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        for label in &self.domain_name {
            bytes.push(label.len() as u8);
            bytes.extend(label.as_bytes());
        }
        bytes.push(0x0);
        bytes.extend(&self.query_type.to_be_bytes());
        bytes.extend(&self.query_class.to_be_bytes());
        bytes
    }

    fn from_bytes(bytes: &[u8]) -> DNSQuestion {
        let mut domain_name = Vec::new();
        let mut current_index = 0;
        while bytes[current_index] != 0x0 {
            let label_length = bytes[current_index] as usize;
            let label = String::from_utf8_lossy(
                &bytes[current_index + 1..current_index + 1 + label_length],
            );
            domain_name.push(label.to_string());
            current_index += 1 + label_length;
        }
        let query_type = u16::from_be_bytes([bytes[current_index + 1], bytes[current_index + 2]]);
        let query_class = u16::from_be_bytes([bytes[current_index + 3], bytes[current_index + 4]]);
        DNSQuestion {
            domain_name,
            query_type,
            query_class,
        }
    }
}

#[derive(Debug)]
pub struct DNSAnswer {
    domain_name: Vec<String>,
    query_type: u16,
    query_class: u16,
    ttl: u32,
    rdlength: u16,
    rdata: Ipv4Addr,
}

impl DNSAnswer {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        for label in &self.domain_name {
            bytes.push(label.len() as u8);
            bytes.extend(label.as_bytes());
        }
        bytes.push(0x0);
        bytes.extend(&self.query_type.to_be_bytes());
        bytes.extend(&self.query_class.to_be_bytes());
        bytes.extend(&self.ttl.to_be_bytes());
        bytes.extend(&self.rdlength.to_be_bytes());
        bytes.extend(&self.rdata.octets());
        bytes
    }
}

#[derive(Debug)]
pub struct DNSQuery {
    header: DNSHeader,
    question_section: DNSQuestion,
}

impl DNSQuery {
    pub fn new(id: u16, question: DNSQuestion) -> DNSQuery {
        DNSQuery {
            header: DNSHeader::new(id, false),
            question_section: question,
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<DNSQuery, ServerError> {
        let header = DNSHeader::from_bytes(bytes)?;
        let question_section = DNSQuestion::from_bytes(&bytes[HEADER_LENGTH..]);
        Ok(DNSQuery {
            header,
            question_section,
        })
    }
}

#[derive(Debug)]
pub struct DNSPacket {
    header: DNSHeader,
    question_section: DNSQuestion,
    answer_section: DNSAnswer,
}

impl DNSPacket {
    pub fn for_request(query: DNSQuery) -> DNSPacket {
        let mut header = DNSHeader::new(query.header.id, true);
        header.qdcount = 1;
        header.opcode = query.header.opcode;
        header.rd = query.header.rd;
        header.rcode = match query.header.opcode {
            0 => 0,
            _ => 4,
        };
        let answer = DNSAnswer {
            domain_name: query.question_section.domain_name.clone(),
            query_type: query.question_section.query_type,
            query_class: query.question_section.query_class,
            ttl: 60,
            rdlength: 4,
            rdata: Ipv4Addr::new(8, 8, 8, 8),
        };
        header.ancount = 1;
        DNSPacket {
            header,
            question_section: query.question_section,
            answer_section: answer,
        }
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = self.header.to_bytes().to_vec();
        bytes.extend(self.question_section.to_bytes());
        bytes.extend(self.answer_section.to_bytes());
        bytes
    }
}
