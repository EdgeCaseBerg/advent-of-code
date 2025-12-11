use std::fs;
use std::time::Instant;
use std::collections::{HashMap, VecDeque};

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
    // Parse into adjacency list: "aaa" => vec!["you", "hhh"]
    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in raw_data.lines().filter(|l| !l.is_empty()) {
        let (left, rest) = line.split_once(':').unwrap();
        let rights: Vec<&str> = rest.split_whitespace().collect();
        graph.insert(left, rights);
    }

    // We're just doing DFS
    let mut stack: Vec<Vec<&str>> = Vec::new();
    stack.push(vec!["you"]);

    let mut count: ResultType = 0;

    while let Some(path) = stack.pop() {
        let last = path[path.len() - 1];

        // If we reached "out", count this path
        if last == "out" {
            count += 1;
            continue;
        }

        // Otherwise explore neighbors
        if let Some(next_nodes) = graph.get(last) {
            for &next in next_nodes {
                if !path.contains(&next) {
                    let mut new_path = path.clone();
                    new_path.push(next);
                    stack.push(new_path);
                }
            }
        }
    }

    count
}


fn p2(_raw_data: &str) -> ResultType {
    0
}