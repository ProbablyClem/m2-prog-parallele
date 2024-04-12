use rayon::prelude::*;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

pub fn read_para() {
    let lines = read_lines("logs.txt").expect("Could not read file");

    lines
        .par_iter()
        .filter(|line| line.to_lowercase().contains("error"))
        .count();
}

pub fn read_serial() {
    let lines = read_lines("logs.txt").expect("Could not read file");

    lines
        .iter()
        .filter(|line| line.to_lowercase().contains("error"))
        .count();
}

pub fn read_lines<P>(filename: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    reader.lines().collect()
}
