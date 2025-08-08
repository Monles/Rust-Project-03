#![allow(unused)]
use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("ferris.txt")
        .expect("Failed to open ferris.txt");

    for _ in 0..5 {
        file.write_all("Ferris\n".as_bytes()).expect("Could not write to ferris.txt");
    }
    for _ in 0..5 {
        file.write_all("Corro\n".as_bytes()).expect("Could not write to ferris.txt");
    }
}