use crc32fast::Hasher;
use byteorder::{LittleEndian, WriteBytesExt, ReadBytesExt};
use std::io::{Cursor, Read};
use std::fmt;

pub struct Record{
    pub header: Header,
    pub key: String,
    pub value: Vec<u8>
}

impl Record  {
    pub fn new(timestamp: u32, key_size: u32, value_size: u32, key: String, value: Vec<u8>, crc: u32) -> Self {
        let header = Header {
            crc,
            timestamp,
            key_size,
            value_size,
        };
        Record { header, key, value }
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut payload = Vec::new();
        payload.write_u32::<LittleEndian>(self.header.crc).unwrap();
        payload.write_u32::<LittleEndian>(self.header.timestamp).unwrap();
        payload.write_u32::<LittleEndian>(self.header.key_size).unwrap();
        payload.write_u32::<LittleEndian>(self.header.value_size).unwrap();
        payload.extend_from_slice(self.key.as_bytes());
        payload.extend_from_slice(&self.value);
        
        payload
    }

    pub fn decode(data: &[u8]) -> Self {
        let mut cursor = Cursor::new(data);
        let crc = cursor.read_u32::<LittleEndian>().unwrap();
        let timestamp = cursor.read_u32::<LittleEndian>().unwrap();
        let key_size = cursor.read_u32::<LittleEndian>().unwrap();
        let value_size = cursor.read_u32::<LittleEndian>().unwrap();

        let mut key_buf = vec![0u8; key_size as usize];
        let mut value = vec![0u8; value_size as usize];

        cursor.read_exact(&mut key_buf).unwrap();
        cursor.read_exact(&mut value).unwrap();

        let key = String::from_utf8(key_buf).unwrap();

        let header = Header {
            crc,
            timestamp,
            key_size,
            value_size,
        };

        Record { header, key, value }
    }

    pub fn compute_crc(data: &[u8]) -> u32 {
        let mut hasher = Hasher::new();
        hasher.update(data);
        hasher.finalize()
    } 
}

pub struct Header{
    crc: u32,
    timestamp: u32,
    key_size: u32,
    value_size: u32,
}

impl fmt::Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Header {{ crc: {}, timestamp: {}, key_size: {}, value_size: {} }}",
            self.crc, self.timestamp, self.key_size, self.value_size
        )
    }
}