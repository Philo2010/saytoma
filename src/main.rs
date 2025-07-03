use std::{cmp, fs::File};

mod ui;
mod raw_reader;



fn main() {
    let mut file = File::open(r"C:\Users\armpe\saytoma\One-Punch Man Chapters 101-105.cbz").expect("AJJJ");

    let hehe = raw_reader::PageReader::new(file).expect("AHHHH");

    println!("{:?}", hehe.paths);
}