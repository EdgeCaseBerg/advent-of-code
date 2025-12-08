use std::fs;
use std::time::Instant;
use std::collections::{HashSet, HashMap};

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

type PointType = i128;
type Tuple3 = (PointType, PointType, PointType); 

fn p1(raw_data: &str) {
    let points: Vec<Tuple3> = raw_data.lines().take_while(|line| !line.is_empty()).map(|line| {
        let mut iter = line.split(",");
        (
            iter.next().expect("Option not defined x").parse::<PointType>().expect("Could not parse number x"),
            iter.next().expect("Option not defined y").parse::<PointType>().expect("Could not parse number y"),
            iter.next().expect("Option not defined z").parse::<PointType>().expect("Could not parse number z")
        )
    }).collect();
    
    let mut circuits: HashMap<Tuple3, HashSet<Tuple3>> = HashMap::new();
    for point in &points {
        let mut shortest_distance = PointType::MAX;
        let mut point_with_shortest_distance = point;
        for other_point in &points {
            // We don't care about ourselves.
            if point == other_point {
                continue;
            }
            let d = euclidean_distance(point, other_point);
            let connections = circuits.entry(*point).or_default();
            let not_in_circuit_yet = !connections.contains(other_point);
            if shortest_distance > d && not_in_circuit_yet {
                shortest_distance = d;
                point_with_shortest_distance = other_point;
            }
        }
        if point != point_with_shortest_distance {
            let connections = circuits.entry(*point).or_default();
            connections.insert(*point_with_shortest_distance);
        }
    }

    // We now have 1 connection between each item to its shortest distance neighbor
    // so now we need to traverse each circuit and compute the three largest circuit
    let mut circuit_sizes = vec!();
    let mut seen: HashSet<&Tuple3> = HashSet::new();
    for (start, _) in &circuits {
        let mut path = vec![start];

        let mut size = 0;
        while let Some(point) = path.pop() {
            seen.insert(point);
            size += 1;
            match circuits.get(point) {
                Some(connections) => {
                    for c in connections.iter() {
                        if !seen.contains(c) {
                            path.push(&c);
                        }
                    }
                },
                None => ()
            };
            
        }
        circuit_sizes.push(size);
    }

    println!("{:?}", circuit_sizes);
    
}

fn euclidean_distance(p1: &Tuple3, p2: &Tuple3) -> PointType {
    let n = (p1.0 - p2.0) * (p1.0 - p2.0) +
    (p1.1 - p2.1) * (p1.1 - p2.1) +
    (p1.2 - p2.2) * (p1.2 - p2.2);
    n.isqrt()
}


fn p2(_raw_data: &str) {}