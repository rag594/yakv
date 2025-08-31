# YAKV - storage engine in Rust

YAKV is built upon bitcask which is a Log-Structured Hash Table for Fast Key/Value Data.It is built on taking inspiration from https://riak.com/assets/bitcask-intro.pdf and https://github.com/mr-karan/barreldb. 

Design philosophy on YAKV:

1. Records in YAKV are stored in little endian format. It is interesting as to why they are stored in such a format.
    - **Streaming parsers**: when reading WAL (Write Ahead Log) files or data files sequentially, you can often decide early whether to skip, buffer, or process a record.
    - **Bounds checking**: if you have memory limits, you can quickly bail out if a `value_size` is too small/large.
    - **Pipelining**: while waiting for the next bytes to arrive from disk, you can already start making partial decisions.
2. Another interesting part is how you write/flush to the disk
    1. When writing the key/value and the whole record to the file/disk, I cannot just directly open a file, write to the buffer and flush for each write ops.
    2. For **high write throughput**, you generally want **batched, buffered, and controlled flushing**, rather than calling `write_all()` + `flush()` on every append. 
    3. In order to solve this we batch the writes in a buffer and then fsync to the disk every N milliseconds or every N requests etc.
    4. One of the tradeoffs is if process crashes before flush then writes are lost.
    5. We are not using nmap and Direct I/O to avoid or bypass read/write syscalls. I’m relying on kernel page cache and want to keep things simpler.