use std::fs;
use std::time::Instant;
use std::collections::HashSet;

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
fn p1(raw_data: &str) -> ResultType {
    let mut input: Vec<(&str, &str)> = Vec::new();
    raw_data.lines().take_while(|line| !line.is_empty()).for_each(|line| {
        let mut parts = line.split(" ");
        let left = parts.next().unwrap().split_once(':').unwrap().0;
        for right in parts {
            input.push((left, right));
        }
    });
    println!("{:?}", input);
    0
}

fn p2(_raw_data: &str) -> ResultType {
    0
}