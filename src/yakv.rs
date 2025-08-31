use crate::data_files::{data_file::*, record::*};
use crate::cache::key_directory::*;

pub struct Yakv {
    pub key_dir_cache: crate::cache::key_directory::KeyDirCache,
    pub data_file: Datafile,
    pub mutex : std::sync::Mutex<()>
}

impl Yakv {
    pub fn new(directory: String, id: u32) -> Self {
        let data_file = Datafile::new(directory, id);
        let key_dir_cache = KeyDirCache::new();
        Yakv { key_dir_cache, data_file, mutex: std::sync::Mutex::new(()) }
    }

    pub fn put(&mut self, timestamp: u32, key: String, value: Vec<u8>) {
        let _lock = self.mutex.lock().unwrap();
        let value_size = value.len() as u32;
        let key_size = key.len() as u32;
        let crc = Record::compute_crc(&value);
        let record = Record::new(timestamp, key_size, value_size, key.clone(), value, crc);
        let record_size = record.encode().len() as u32;
        let record_pos = self.data_file.write(record);
        let metadata = KeyDirectoryMetadata {
            file_id: self.data_file.id,
            record_size,
            position: record_pos + record_size,
            timestamp
        };
        self.key_dir_cache.insert(key, metadata);
    }

    pub fn get(&mut self, key: &String) -> Option<Record> {
        let _lock = self.mutex.lock().unwrap();
        if let Some(metadata) = self.key_dir_cache.get(key) {
            let record = self.data_file.read(metadata.position, metadata.record_size);
            Some(record)
        } else {
            None
        }
    }
}