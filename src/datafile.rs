use std::os::unix::fs::MetadataExt;
use std::io::{Write, Seek, Read};
use std::path::Path;
use crate::record;

pub struct Datafile {
    offset: u32,
    id: u32,
    file: std::fs::File
}



impl Datafile {
    pub fn new(directory: String, id: u32) -> Self {
        let path: String = format!("{}/YAKV_{}.db", directory, id);
        let p = Path::new(&path);
        let _file = std::fs::OpenOptions::new().read(true).append(true).create(true).open(p).unwrap();
        let size = std::fs::metadata(p).unwrap().size() as u32;
        Datafile { offset: size, id, file: _file}
    }

    pub fn write(&mut self, record: record::Record) -> u32 {
        let encoded = record.encode();
        let pos = self.offset;
        self.file.write_all(&encoded).unwrap();
        self.offset += encoded.len() as u32;
        pos
    }

    pub fn read(&mut self, pos: u32, size: u32) -> record::Record {
        let mut buffer = vec![0; size as usize];
        self.file.seek(std::io::SeekFrom::Start(pos as u64)).unwrap();
        self.file.read_exact(&mut buffer).unwrap();
        record::Record::decode(&buffer)
    }
    
}