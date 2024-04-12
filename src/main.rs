use std::time::{Duration, Instant};

mod read_file;
// Function to read a file
fn main() {
    // We read the file a first time to warm up the cache

    let start = Instant::now();
    read_file::read_lines("logs.txt").expect("Could not read file");
    let duration = start.elapsed();
    println!("Time elapsed warmup is: {:?}", duration);

    let nb_iter = 10;
    let serial_mean = (0..nb_iter)
        .map(|_| bench_serial())
        .fold(Duration::ZERO, |acc, x| acc + x)
        / nb_iter;

    println!(
        "Mean time elapsed in read_file_serial() is: {:?}",
        serial_mean
    );

    let parallel_mean = (0..nb_iter)
        .map(|_| bench_parallel())
        .fold(Duration::ZERO, |acc, x| acc + x)
        / nb_iter;
    println!(
        "Mean time elapsed in read_file_parallel() is: {:?}",
        parallel_mean
    );
}

fn bench_serial() -> std::time::Duration {
    let start = Instant::now();
    read_file::read_serial();
    start.elapsed()
}

fn bench_parallel() -> std::time::Duration {
    let start = Instant::now();
    read_file::read_para();
    start.elapsed()
}
