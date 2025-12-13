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
struct Shape {
    index: u8,
    shape: [[usize; 3]; 3]
}

impl From<(usize, &str)> for Shape {
    fn from(tuple: (usize, &str)) -> Shape {
        let (idx, string) = tuple;
        let mut s = Shape {
            index: idx as u8,
            shape: [
                [0, 0, 0],
                [0, 0, 0],
                [0, 0, 0]
            ]
        };

        // Blog note: don't forget to use ! in front of line.is_empty 
        let lines = string
            .lines()
            .take_while(|line| !line.is_empty())
            .enumerate();

        for (row, line) in lines {
            for c in 0..3 {
                s.shape[row][c] = if let Some(ch) = line.chars().nth(c) {
                    if ch == '#' { 1 } else { 0 }
                } else { 0 }
            }
        }

        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let full: Shape = (0, "###\n###\n###").into();
        assert_eq!(full.shape, [[1,1,1],[1,1,1],[1,1,1]]);

        let partial: Shape = (0, ".##\n#.#\n##.").into();
        assert_eq!(partial.shape, [[0,1,1],[1,0,1],[1,1,0]]);

        let none: Shape = (0, "...\n...\n...").into();
        assert_eq!(none.shape, [[0,0,0],[0,0,0],[0,0,0]]);
    }

}

struct Region {
    width: u8,
    height: u8,
    quantity_to_fit_per_shape: [usize; 6]
}

// TODO: define rotate/flip functions, overlaps?



fn p2(_raw_data: &str) -> ResultType {
    0
}