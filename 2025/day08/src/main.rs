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

type Tuple3 = (usize, usize, usize); 

fn p1(raw_data: &str) {
    let points: Vec<Tuple3> = raw_data.lines().take_while(|line| !line.is_empty()).map(|line| {
        let mut iter = line.split(",");
        (
            iter.next().expect("Option not defined x").parse::<usize>().expect("Could not parse number x"),
            iter.next().expect("Option not defined y").parse::<usize>().expect("Could not parse number y"),
            iter.next().expect("Option not defined z").parse::<usize>().expect("Could not parse number z")
        )
    }).collect();
    println!("{:?}", points);
    println!("{:?}", euclidean_distance(points[0], points[1]));
}

fn euclidean_distance(p1:Tuple3, p2: Tuple3) -> usize {
    let n = (p1.0 - p2.0) * (p1.0 - p2.0) +
    (p1.1 - p2.1) * (p1.1 - p2.1) +
    (p1.2 - p2.2) * (p1.2 - p2.2);
    n.isqrt()
}


fn p2(_raw_data: &str) {}