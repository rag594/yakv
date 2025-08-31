pub struct Yakv {
    pub key_dir_cache: crate::key_directory::KeyDirCache,
    pub data_file: crate::data_file::Datafile
}

impl Yakv {
    pub fn new(directory: String, id: u32) -> Self {
        let data_file = crate::data_file::Datafile::new(directory, id);
        let key_dir_cache = crate::key_directory::KeyDirCache::new();
        Yakv { key_dir_cache, data_file }
    }

    pub fn put(&mut self, timestamp: u32, key: String, value: Vec<u8>) {
        let value_size = value.len() as u32;
        let key_size = key.len() as u32;
        let crc = crate::record::Record::compute_crc(&value);
        let record = crate::record::Record::new(timestamp, key_size, value_size, key.clone(), value, crc);
        let record_size = record.encode().len() as u32;
        let record_pos = self.data_file.write(record);
        let metadata = crate::key_directory::KeyDirectoryMetadata {
            file_id: self.data_file.id,
            record_size,
            position: record_pos + record_size,
            timestamp
        };
        self.key_dir_cache.insert(key, metadata);
    }

    pub fn get(&mut self, key: &String) -> Option<crate::record::Record> {
        if let Some(metadata) = self.key_dir_cache.get(key) {
            println!("Metadata found for key {}: file_id={}, record_size={}, position={}, timestamp={}", key, metadata.file_id, metadata.record_size, metadata.position, metadata.timestamp);
            let record = self.data_file.read(metadata.position, metadata.record_size);
            Some(record)
        } else {
            None
        }
    }
}