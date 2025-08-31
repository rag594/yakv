use std::time::{SystemTime, UNIX_EPOCH};
mod record;
mod data_file;
mod key_directory;
mod yakv;


fn main() {
    let mut yakv = crate::yakv::Yakv::new(".".to_string(), 1);

    let now = SystemTime::now().duration_since(UNIX_EPOCH).expect("time went backwards").as_secs() as u32;
    yakv.put(now, "hello".to_string(), "woerrr".as_bytes().to_vec());
    yakv.put(now, "apple".to_string(), "jeqkhkh".as_bytes().to_vec());


    yakv.get(&"hello".to_string()).map(|record| {
        println!("Got Record: key = {}, value = {:?}", record.key, String::from_utf8(record.value).unwrap());
    });

    yakv.get(&"apple".to_string()).map(|record| {
        println!("Got Record: key = {}, value = {:?}", record.key, String::from_utf8(record.value).unwrap());
    });
}
