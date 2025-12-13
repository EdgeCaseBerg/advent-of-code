use std::fs;
use std::time::Instant;

fn main() {
    let raw_data = fs::read_to_string("./input").expect("bad input data");
    let raw_data = raw_data.as_str();
    let i = Instant::now();
    let result = p1(raw_data);
    let time = i.elapsed();
    println!("{:?}", result);
    println!("Took: {:?}", time);

    let i = Instant::now();
    let result = p2(raw_data);
    let time = i.elapsed();
    println!("{:?}", result);
    println!("Took: {:?}", time);
}


type ResultType = i64;
fn p1(_raw_data: &str) -> ResultType {
    0
}

#[derive(Debug)]
struct Shape<'a> {
    index: u8,
    shape: [[usize; 3]; 3]
}

// TODO: define rotate/flip functions and such


fn p2(_raw_data: &str) -> ResultType {
    0
}