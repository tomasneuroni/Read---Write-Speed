use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::time::Instant;

const BUFFER_SIZE: usize = 1024 * 1024;

fn main() {
    let file = File::create("test.bin").unwrap();
    let mut writer = BufWriter::new(file);

    let start = Instant::now();
    for _ in 0..100 {
        writer.write_all(&[0; BUFFER_SIZE]).unwrap();
    }
    let write_speed = 100.0 * BUFFER_SIZE as f64 / start.elapsed().as_secs_f64();

    let file = File::open("test.bin").unwrap();
    let mut reader = BufReader::new(file);
    let mut buffer = vec![0; BUFFER_SIZE];

    let start = Instant::now();
    for _ in 0..100 {
        reader.read_exact(&mut buffer).unwrap();
    }
    let read_speed = 100.0 * BUFFER_SIZE as f64 / start.elapsed().as_secs_f64();

    println!("Read speed: {:.2} MB/s", read_speed / 1024.0 / 1024.0);
    println!("Write speed: {:.2} MB/s", write_speed / 1024.0 / 1024.0);
}
