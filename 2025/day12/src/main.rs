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
fn p1(raw_data: &str) -> ResultType {
    let shapes: Vec<Shape> = raw_data.split("\n\n").take(6).map(Shape::from).collect();
    println!("{:?}", shapes);
    // How many regions can fit the presents listed?
    0
}

#[derive(Debug)]
struct Shape {
    index: u8,
    shape: [[usize; 3]; 3]
}

impl From<&str> for Shape {
    // Input is of the form, index:\n...\n###\n.... where # and . indicate shape or not
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
mod test_shapes {
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

impl From<&str> for Region {
    fn from(string: &str) -> Region {
        let (sizing, numbers) = string.split_once(":").unwrap();
        let (width, height) = sizing.split_once("x").unwrap();

        let mut counts = [0usize; 6];
        let numbers = numbers
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .take(6)
            .enumerate();
        for (i, n) in numbers {
            counts[i] = n;
        }

        Region {
            width: width.parse().unwrap(),
            height: height.parse().unwrap(),
            quantity_to_fit_per_shape: counts
        }
    }
}

#[cfg(test)]
mod test_regions {
    use super::*;

    #[test]
    fn test_parse() {
        let region: Region = "40x42: 38 37 45 42 54 41".into();
        assert_eq!(region.width, 40);
        assert_eq!(region.height, 42);
        assert_eq!(region.quantity_to_fit_per_shape, [38,37,45,42,54,41]);

        let region: Region = "4x4: 38 37 45 42 54 41".into();
        assert_eq!(region.width, 4);
        assert_eq!(region.height, 4);
        assert_eq!(region.quantity_to_fit_per_shape, [38,37,45,42,54,41]);
    }

}



// TODO: define rotate/flip functions, overlaps?



fn p2(_raw_data: &str) -> ResultType {
    0
}