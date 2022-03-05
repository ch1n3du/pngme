// Implementation of PNG chunk type [http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html]
use std::fmt;

use crate::chunk_type::*;

pub struct Chunk {
    length: u32,
    chunk_type: ChunkType,
    chunk_data: [u8],
    crc: u32
}

pub fn vecToArr(vec: Vec<T>) -> Option<[T]> {
    match vec.as_slice().try_into() {
        Ok(arr) => Some(arr),
        Err(_) => None
    }
}

// TODO: Write func to calculate crc of given
pub fn getCrc(chunk_type: &ChunkType, chunk_data: &[u8]) -> u32 {

    let temp_vec = chunk_type.bytes.iter()
        .chain(chunk_data.iter())
        .collect();

    let important_bytes = vecToArr(temp_vec).unwrap();

    // TODO: Get crc of important bytes
    todo!()
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        if let Some(chunk_data) = vecToArr<u8>(data) {

            let crc = getCrc(&chunk_type, &chunk_data);

            Chunk {
                length: data.len(),
                chunk_type,
                chunk_data,
                crc 
            }
        }

        panic!("Invalid Data Bytes");
    }

    pub fn length(&self) -> u32 {
        self.length.to_be_bytes()
    }

    pub fn chunk_type(&self) -> &ChunkType {
        self.chunk_type
    }

    pub fn data(&self) -> &[u8] {
        self.chunk_data
    }

    pub fn crc(&self) -> u32 {
        self.crc.to_be_bytes()
    }

    pub fn data_as_string(&self) -> Result<String> {
        String::from_utf8(self.chunk_data)
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.length.to_be_bytes()
            .iter()
            .chain(self.chunk_type.bytes().iter())
            .chain(self.chunk_data.iter())
            .chain(self.crc.to_be_bytes().iter())
            .collect()
    }
}

pub enum Error {
    InsufficientBytes
}

impl TryFrom<&[u8]> for Chunk {
    type Error = Error;

    fn try_from(bytes: &[u8]) -> Result<Chunk, Error> {
        let len_bytes = bytes.len();

        if bytes.len() < 12 {
            Err(Error::InsufficientBytes)
        } else {
            todo!()
        }
    }
}

impl fmt::Display for Chunk {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Formatter {
        write!(f, "Chunk_Type: {}\nLength: {}\nCRC: {}\nChunk_Data: {}"
            , self.chunk_type
            , self.length
            , self.crc
            , self.chunk_data)
    }
}