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
    let num_connections = 1000;
    let points: Vec<Tuple3> = raw_data.lines().take_while(|line| !line.is_empty()).map(|line| {
        let mut iter = line.split(",");
        (
            iter.next().expect("Option not defined x").parse::<PointType>().expect("Could not parse number x"),
            iter.next().expect("Option not defined y").parse::<PointType>().expect("Could not parse number y"),
            iter.next().expect("Option not defined z").parse::<PointType>().expect("Could not parse number z")
        )
    }).collect();
    
    let mut circuits: HashMap<Tuple3, HashSet<Tuple3>> = HashMap::new();
    // It's not the pairwise of shortest distances! ...
    // for point in &points {
    //     let mut shortest_distance = PointType::MAX;
    //     let mut point_with_shortest_distance = point;
    //     for other_point in &points {
    //         // We don't care about ourselves.
    //         if point == other_point {
    //             continue;
    //         }
    //         let d = euclidean_distance_squared(point, other_point);
    //         let connections = circuits.entry(*point).or_default();
    //         let not_in_circuit_yet = !connections.contains(other_point);
    //         if shortest_distance > d && not_in_circuit_yet {
    //             shortest_distance = d;
    //             point_with_shortest_distance = other_point;
    //         }
    //     }
    //     if point != point_with_shortest_distance {
    //         let connections = circuits.entry(*point).or_default();
    //         connections.insert(*point_with_shortest_distance);
    //     }
    // }
    // // We now have 1 connection between each item to its shortest distance neighbor
    // // so now we need to traverse each circuit and compute the three largest circuit
    // let mut circuit_sizes = vec!();
    // for (start, _) in &circuits {
    //     let mut seen: HashSet<&Tuple3> = HashSet::new();
    //     let mut path = vec![start];

    //     let mut size = 1;
    //     while let Some(point) = path.pop() {
    //         seen.insert(point);
    //         match circuits.get(point) {
    //             Some(connections) => {
    //                 for c in connections.iter() {
    //                     if !seen.contains(c) {
    //                         println!("{:?} -> {:?}", point, c);
    //                         size += 1;
    //                         path.push(&c);
    //                     }
    //                 }
    //             },
    //             None => ()
    //         };
            
    //     }
    //     circuit_sizes.push(size);
    // }
    // ... its supposed to be a global list of shortest distances between pairs

    // Step 1: map points to indices
    let mut index = HashMap::<Tuple3, usize>::new();
    for (i, p) in points.iter().enumerate() {
        index.insert(*p, i);
    }

    // Treat tuple connections as graph edges
    let mut edges = vec![];
    for (i, a) in points.iter().enumerate() {
        for (j, b) in points.iter().enumerate().skip(i + 1) {
            edges.push((euclidean_distance_squared(a, b), i, j));
        }
    }

    // And sort it all by distance
    edges.sort_by(|a, b| {
        a.0.cmp(&b.0)
            .then_with(|| points[a.1].cmp(&points[b.1]))
            .then_with(|| points[a.2].cmp(&points[b.2]))
    });

    // The DSU starts with a single point.
    // then we start adding in each edge
    let mut dsu = DSU::new(points.len());
    let mut added = 0;
    let mut attempts = 0;
    for (_, i, j) in edges {
        // this is 10 in the example, 1000 in the problem
        // and if you screw this up then you get the wrong number.
        if attempts == num_connections {
            break;
        }
        attempts += 1;
        dsu.union(i, j);
    }


    // count each circuit's size
    let mut sizes = Vec::new();
    let mut seen = HashSet::new();

    for i in 0..points.len() {
        let root = dsu.find(i);
        if seen.insert(root) {
            sizes.push(dsu.size[root]);
        }
    }

    sizes.sort_by(|a, b| b.cmp(a));
    let result = sizes[0] * sizes[1] * sizes[2];

    println!("Largest 3 circuits: {:?}", &sizes[..3]);
    println!("Product: {}", result);
}

// for sorting purposes, squared and not squared work just fine.
fn euclidean_distance_squared(p1: &Tuple3, p2: &Tuple3) -> PointType {
    (p1.0 - p2.0) * (p1.0 - p2.0) +
    (p1.1 - p2.1) * (p1.1 - p2.1) +
    (p1.2 - p2.2) * (p1.2 - p2.2)
}

// Fancy little thing called a Disjoint Set
#[derive(Debug)]
struct DSU {
    parent: Vec<usize>,
    size: Vec<usize>,
}

/**
 * Union-Find (Disjoint Set) Data Structure
 * 
 * Tracks which elements belong to the same group (circuit).
 * 
 * - All elements in the same set have the same root
 * - Path compression: flattens trees during traversal for speed
 * - Union by rank: always attaches smaller tree to larger tree
 */
impl DSU {
    fn new(n: usize) -> Self {
        DSU {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]); // path compression
        }
        self.parent[x]
    }

    fn union(&mut self, a: usize, b: usize) -> bool {
        let mut a = self.find(a);
        let mut b = self.find(b);

        if a == b {
            return false;
        }

        // union by size
        if self.size[a] < self.size[b] {
            std::mem::swap(&mut a, &mut b);
        }

        self.parent[b] = a;
        self.size[a] += self.size[b];
        true
    }

    fn component_size(&mut self, x: usize) -> usize {
        let root = self.find(x);
        self.size[root]
    }
}



fn p2(_raw_data: &str) {}