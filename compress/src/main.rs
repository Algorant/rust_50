extern crate flate2;

use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;
use std::fs::File;
use std::io::{copy, BufReader};
use std::time::Instant;
// use std::fmt::eprintln;

// Helper function to conver to MB
fn bytes_to_mb(bytes: u64) -> f64 {
    bytes as f64 / (1024.0 * 1024.0)
}

fn main() {
    if args().len() != 3 {
        eprintln!("Usage: `source` `target`");
        return;
    }
    let mut input = BufReader::new(File::open(args()
        .nth(1)
        .unwrap())
        .unwrap());

    let output = File::create(args()
        .nth(2)
        .unwrap())
        .unwrap();

    let mut encoder = GzEncoder::new(output, Compression::default());

    let start = Instant::now();

    copy(&mut input, &mut encoder).unwrap();

    let output = encoder
        .finish()
        .unwrap();

    let source_len = input.get_ref().metadata().unwrap().len();
    let target_len = output.metadata().unwrap().len();

    // Print the outputs in readable format
    println!("Source len:{:.2} MB", bytes_to_mb(source_len));
    println!("Target len:{:.2} MB", bytes_to_mb(target_len));
    println!("Elapsed: {:.3} seconds", start.elapsed().as_secs_f64());

}
