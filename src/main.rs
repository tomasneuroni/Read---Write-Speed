use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::time::{Duration, Instant};
use std::thread::sleep;

const MB: u64 = 1024 * 1024;

fn main() {
    println!("Enter the number of seconds you would like to run the test for:");
    let mut duration = String::new();
    std::io::stdin().read_line(&mut duration).unwrap();
    let duration = duration.trim().parse::<u64>().unwrap();

    println!("Running disk speed test for {} seconds...", duration);

    let filename = "test.txt";
    let data_size = MB * 100;

    let start = Instant::now();
    let end = start + Duration::from_secs(duration);

    loop {
        let now = Instant::now();
        if now >= end {
            break;
        }

        println!("Writing data to disk...");
        let write_start = Instant::now();
        let mut file = OpenOptions::new().write(true).create(true).open(filename).unwrap();
        let data = vec![0u8; data_size as usize];
        file.write_all(&data).unwrap();
        file.flush().unwrap();
        let write_elapsed = write_start.elapsed();

        println!("Reading data from disk...");
        let read_start = Instant::now();
        let mut file = File::open(filename).unwrap();
        let mut data = vec![0u8; data_size as usize];
        file.read_exact(&mut data).unwrap();
        let read_elapsed = read_start.elapsed();

        let write_speed = (data_size as f64) / (write_elapsed.as_secs_f64());
        let read_speed = (data_size as f64) / (read_elapsed.as_secs_f64());

        println!("Write speed: {:.2} MB/s", write_speed / MB as f64);
        println!("Read speed: {:.2} MB/s", read_speed / MB as f64);
        println!("");

        sleep(Duration::from_secs(1));
    }

    println!("Finished disk speed test");
    std::fs::remove_file(filename).unwrap();
}
