use rayon::prelude::*;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn read_file(filename: &str) -> io::Result<BufReader<File>> {
    let file = File::open(filename)?;
    Ok(BufReader::new(file))
}

pub fn read_para() {
    let target_word = "error";

    let reader = read_file("logs.txt").expect("Could not read file");

    let _error_count: usize = reader
        .lines()
        .filter_map(Result::ok)
        .par_bridge()
        .filter(|line| {
            line.contains(target_word) // This is a stand in  for expensive line processing
        })
        .count();
}

pub fn read_serial() {
    let target_word = "error";

    let reader = read_file("logs.txt").expect("Could not read file");

    let _error_count: usize = reader
        .lines()
        .filter_map(Result::ok)
        .filter(|line| {
            line.contains(target_word) // This is a stand in  for expensive line processing
        })
        .count();
}
