pub struct KeyDirectoryMetadata {
    pub file_id: u32,
    pub record_size: u32,
    pub position: u32,
    pub timestamp: u32
}

pub struct KeyDirCache {
    map: std::collections::HashMap<String, KeyDirectoryMetadata>
}

impl KeyDirCache {
    pub fn new() -> Self {
        KeyDirCache { map: std::collections::HashMap::new() }
    }

    pub fn insert(&mut self, key: String, metadata: KeyDirectoryMetadata) {
        self.map.insert(key, metadata);
    }

    pub fn get(&self, key: &String) -> Option<&KeyDirectoryMetadata> {
        self.map.get(key)
    }
}

