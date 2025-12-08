use std::fs;
use std::time::Instant;

fn main() {
    let raw_data = fs::read_to_string("./input").expect("bad input data");
    let raw_data = raw_data.as_str();
    let i = Instant::now();
    p1(raw_data);
    println!("{:?}", i.elapsed());

    let i = Instant::now();
    p2(raw_data);
    println!("{:?}", i.elapsed());
}

fn p1(_raw_data: &str) {
}

fn p2(_raw_data: &str) {}