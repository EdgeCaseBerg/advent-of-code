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

impl From<&str> for Shape {
    fn from(string: &str) -> Shape {

        // Blog note: don't forget to use ! in front of line.is_empty 
        let mut lines = string
            .lines()
            .take_while(|line| !line.is_empty())
            .enumerate();

        let idx = lines.next().unwrap().1.split_once(":").unwrap().0;

        let mut s = Shape {
            index: idx.parse().unwrap(),
            shape: [
                [0, 0, 0],
                [0, 0, 0],
                [0, 0, 0]
            ]
        };

        for (row, line) in lines {
            for c in 0..3 {
                s.shape[row - 1][c] = if let Some(ch) = line.chars().nth(c) {
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
        let full: Shape = "0:\n###\n###\n###".into();
        assert_eq!(full.shape, [[1,1,1],[1,1,1],[1,1,1]]);
        assert_eq!(full.index, 0);

        let partial: Shape = "1:\n.##\n#.#\n##.".into();
        assert_eq!(partial.shape, [[0,1,1],[1,0,1],[1,1,0]]);
        assert_eq!(partial.index, 1);

        let none: Shape = "2:\n...\n...\n...".into();
        assert_eq!(none.shape, [[0,0,0],[0,0,0],[0,0,0]]);
        assert_eq!(none.index, 2);
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