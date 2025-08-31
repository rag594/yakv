use std::time::{SystemTime, UNIX_EPOCH};
mod record;
mod datafile;
use crate::{datafile::Datafile, record::Record};


fn main() {

    let mut data_file = Datafile::new(".".to_string(), 1);

    let now = SystemTime::now().duration_since(UNIX_EPOCH).expect("time went backwards").as_secs() as u32;
    let key = "hello".to_string();
    let value = "world".as_bytes().to_vec();
    let crc = record::Record::compute_crc(value.as_slice());

    let record: Record = record::Record::new(now, key.len() as u32, value.len() as u32, key, value, crc);

    let record_size = record.encode().len() as u32;

    let record_pos = data_file.write(record);

    print!("Written record at position: {}, size: {}\n", record_pos, record_size);

    let read_record = data_file.read(record_pos, record_size);
    println!("Read Record: key = {}, value = {:?}", read_record.key, String::from_utf8(read_record.value).unwrap());
}
