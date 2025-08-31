use std::os::unix::fs::MetadataExt;
use std::path::Path;
use crate::data_files::record;
use crate::data_files::buffered_file::BufferedFile;

pub struct Datafile {
    pub offset: u32,
    pub id: u32,
    pub buffered_file: BufferedFile
}



impl Datafile {
    pub fn new(directory: String, id: u32) -> Self {
        let path: String = format!("{}/YAKV_{}.db", directory, id);
        let p = Path::new(&path);
        let buffered_file = BufferedFile::open(p).unwrap();
        let size = std::fs::metadata(p).unwrap().size() as u32;
        Datafile { offset: size, id, buffered_file  }
    }

    pub fn write(&mut self, record: record::Record) -> u32 {
        let encoded = record.encode();
        let pos = self.offset;
        self.buffered_file.append(&encoded).unwrap();
        self.buffered_file.flush().unwrap(); // TODO: this needs to be done in a background thread to avoid frequent disk writes
        self.offset += encoded.len() as u32;
        pos
    }

    pub fn read(&mut self, pos: u32, size: u32) -> record::Record {
        let mut buffer = vec![0; size as usize];
        let start = pos - size;
        self.buffered_file.read_at(start as u64, &mut buffer).unwrap();
        record::Record::decode(&buffer)
    }
    
}