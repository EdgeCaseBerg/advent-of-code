use std::fs;
use std::cmp::max;

fn main() {
    let raw_data = fs::read_to_string("./input").expect("bad input data");
    let raw_data = raw_data.as_str();
    p1(raw_data);
    p2(raw_data);
}

fn p1(raw_data: &str) {
    let (matrix, rows, cols) = make_matrix(raw_data);
    let s_location = matrix[0].iter().position(|c| *c == 'S').expect("Could not find start in first row");
    let mut beams = vec![s_location];
    let mut number_of_splits = 0;
    for r in 1..rows {
        for c in 0..cols {
            // A beam in the current list of beams has hit a splitter
            if matrix[r][c] == '^' && beams.contains(&c) {
                number_of_splits += 1;
                beams.retain(|idx| *idx != c);
                if c > 0 {
                    beams.push(c - 1);
                }
                if c < cols {
                    beams.push(c + 1);
                }
                // The problem makes a point of noting merges so, make sure we
                // nix double counts as needed.
                beams.sort();
                beams.dedup();
            }
        }
    }
    println!("{:?}", number_of_splits);
}

fn p2(raw_data: &str) {
    let (matrix, rows, cols) = make_matrix(raw_data);

    // Draw the world lines, this will give us our graph.
    let mut worldlines: Vec<Vec<usize>> = Vec::with_capacity(rows);
    for r in 0..rows {
        let mut tmp: Vec<usize> = Vec::with_capacity(cols);
        for c in 0..cols {
            tmp.push(0);
        }
        worldlines.push(tmp);
    }

    let s_location = matrix[0].iter().position(|c| *c == 'S').expect("Could not find start in first row");

    worldlines[0][s_location] = 1;
    let mut beams = vec![s_location];

    for r in 1..rows {
        for c in 0..cols {
            // A beam in the current list of beams has hit a splitter
            if matrix[r][c] == '^' && beams.contains(&c) {
                let how_many_lead_to_this_beam = worldlines[r - 1][c];
                println!("hmlttb {:?}", how_many_lead_to_this_beam);
                beams.retain(|idx| *idx != c);
                if c > 0 {
                    beams.push(c - 1);
                    worldlines[r][c - 1] += how_many_lead_to_this_beam;
                }
                if c < cols {
                    beams.push(c + 1);
                    worldlines[r][c + 1] += how_many_lead_to_this_beam;
                }
                beams.sort();
                beams.dedup();
            } else if beams.contains(&c) {
                worldlines[r][c] = worldlines[r - 1][c];
            }
        }
        for c in beams.iter() {
            worldlines[r][*c] = max(worldlines[r - 1][*c], 1);
        }
    }

    // Now we must traverse the graph intelligently to count how many paths there are through it.
    for line in worldlines {
        println!("{:?}", line);
    }
    /* Does this help me? Could I use it as input to myself to count unique paths in
       this "maze" ? Or does the fact that we have multiple 'exits' pose a problem to
       that idea
    ['.', '.', '.', '.', '.', '.', '.', 'S', '.', '.', '.', '.', '.', '.', '.'] 1
    ['.', '.', '.', '.', '.', '.', '.', '|', '.', '.', '.', '.', '.', '.', '.']
    ['.', '.', '.', '.', '.', '.', '|', '^', '|', '.', '.', '.', '.', '.', '.'] 2
    ['.', '.', '.', '.', '.', '.', '|', '.', '|', '.', '.', '.', '.', '.', '.']
    ['.', '.', '.', '.', '.', '|', '^', '|', '^', '|', '.', '.', '.', '.', '.'] 4 = 1 + 2 + 1
    ['.', '.', '.', '.', '.', '|', '.', '|', '.', '|', '.', '.', '.', '.', '.']
    ['.', '.', '.', '.', '|', '^', '|', '^', '|', '^', '|', '.', '.', '.', '.'] 8 = 1 + 3 + 3 + 1
    ['.', '.', '.', '.', '|', '.', '|', '.', '|', '.', '|', '.', '.', '.', '.']
    ['.', '.', '.', '|', '^', '|', '^', '|', '|', '|', '^', '|', '.', '.', '.'] 9 = 1 + 4 + 3 + 1 
    ['.', '.', '.', '|', '.', '|', '.', '|', '|', '|', '.', '|', '.', '.', '.']
    ['.', '.', '|', '^', '|', '^', '|', '|', '|', '^', '|', '^', '|', '.', '.']
    ['.', '.', '|', '.', '|', '.', '|', '|', '|', '.', '|', '.', '|', '.', '.']
    ['.', '|', '^', '|', '|', '|', '^', '|', '|', '.', '|', '|', '^', '|', '.']
    ['.', '|', '.', '|', '|', '|', '.', '|', '|', '.', '|', '|', '.', '|', '.']
    ['|', '^', '|', '^', '|', '^', '|', '^', '|', '^', '|', '|', '|', '^', '|']
    ['|', '.', '|', '.', '|', '.', '|', '.', '|', '.', '|', '|', '|', '.', '|']
      If you have 9 sinks, and 1 source, then can you follow it them back up and
      solve each case, caching results along the way, to figure it out? But then
      how we count the actual timelines possible. Should each splitter keep track
      of how many routes lead into it? ... Let's consider it at each junction instead
      maybe.
        [0, 0, 0, 0, 0, 0, 0, S, 0, 0, 0, 0, 0, 0, 0]
        [0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0]
        [0, 0, 0, 0, 0, 0, 1, ^, 1, 0, 0, 0, 0, 0, 0]
        [0, 0, 0, 0, 0, 1, ^, 2, ^, 1, 0, 0, 0, 0, 0]
        [0, 0, 0, 0, 0, 1, 0, 2, 0, 1, 0, 0, 0, 0, 0]
        [0, 0, 0, 0, 0, 1, 0, 2, 0, 1, 0, 0, 0, 0, 0]
        [0, 0, 0, 0, 1, ^, 3, ^, 3, ^, 1, 0, 0, 0, 0]
        [0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0]
        [0, 0, 0, 1, 0, 1, 0, 1, 1, 1, 0, 1, 0, 0, 0]
        [0, 0, 0, 1, 0, 1, 0, 1, 1, 1, 0, 1, 0, 0, 0]
        [0, 0, 1, 0, 1, 0, 1, 1, 1, 0, 1, 0, 1, 0, 0]
        [0, 0, 1, 0, 1, 0, 1, 1, 1, 0, 1, 0, 1, 0, 0]
        [0, 1, 0, 1, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 0]
        [0, 1, 0, 1, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 0]
        [1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 1, 1, 0, 1]
        [1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 1, 1, 0, 1]

     */
    // let number_of_world_lines = microwave_banana(&matrix, rows, cols, 0, s_location);
    // println!("{:?}", number_of_world_lines);
}

// fn microwave_banana(raw_data: &Vec<Vec<char>>, rows: usize, cols: usize, row: usize, col: usize) {
// }

fn make_matrix(raw_data: &str) -> (Vec<Vec<char>>, usize, usize) {
        // Parse the matrix
    let matrix: Vec<Vec<char>> = raw_data
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect();

    let rows = matrix.len();
    let cols = matrix.get(0).map(|m| m.len()).unwrap_or(0);
    (matrix, rows, cols)
}