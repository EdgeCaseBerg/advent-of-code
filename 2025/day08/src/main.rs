use std::fs;
use std::time::Instant;
use std::collections::{HashSet, HashMap};

// This is the trick: https://en.wikipedia.org/wiki/Kruskal%27s_algorithm
fn main() {
    let raw_data = fs::read_to_string("./input").expect("bad input data");
    let raw_data = raw_data.as_str();
    let i = Instant::now();
    p1(raw_data);
    println!("Took: {:?}", i.elapsed());

    let i = Instant::now();
    p2(raw_data);
    println!("Took: {:?}", i.elapsed());
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
    let mut attempts = 0;
    for (_, i, j) in edges {
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
}


fn p2(raw_data: &str) {
    let points: Vec<Tuple3> = raw_data.lines().take_while(|line| !line.is_empty()).map(|line| {
        let mut iter = line.split(",");
        (
            iter.next().expect("Option not defined x").parse::<PointType>().expect("Could not parse number x"),
            iter.next().expect("Option not defined y").parse::<PointType>().expect("Could not parse number y"),
            iter.next().expect("Option not defined z").parse::<PointType>().expect("Could not parse number z")
        )
    }).collect();
    
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
    let mut merges = 0;
    let mut final_merge: Option<(Tuple3, Tuple3)> = None;
    for (_, i, j) in edges {
        let merged = dsu.union(i, j);
        if merged {
            merges += 1;
        }
        if merges == points.len() -1 {
            final_merge = Some((points[i], points[j]));
            break;
        }
    }

    let final_merge = final_merge.expect("Didnt compute final merge?");
    println!("final merge X {:?}", final_merge.0.0 * final_merge.1.0);
}

