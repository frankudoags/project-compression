extern crate flate2;

#[allow(unused_imports)]
use flate2::write::GzEncoder;
#[allow(unused_imports)]
use flate2::Compression;
#[allow(unused_imports)]
use std::{
    env::args,
    fs::File,
    io::{copy, prelude::*, BufReader},
    time::Instant,
};

fn main() {
    if args().len() != 3 {
        println!("No input file or output file specified");
        println!("Usage:  <input file> <output file>");
        return;
    }

    let mut input =
        BufReader::new(File::open(args().nth(1).unwrap()).expect("Failed to open input file"));

    let output = File::create(args().nth(2).unwrap()).expect("Failed to create output file");

    let start = Instant::now();

    let mut encoder = GzEncoder::new(&output, Compression::default());

    copy(&mut input, &mut encoder).expect("Failed to compress file");

    encoder.finish().expect("Failed to finish compression");

    println!(
        "Input file size: {} bytes",
        input.get_ref().metadata().unwrap().len()
    );
    println!(
        "Ouput file size: {} bytes",
        output.metadata().unwrap().len()
    );
    println!(
        "Compression ratio: {} %",
        (output.metadata().unwrap().len() as f64
            / input.get_ref().metadata().unwrap().len() as f64)
            * 100.0
    );
    println!("Compression took {} seconds", start.elapsed().as_secs());
}
