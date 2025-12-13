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
    let regions: Vec<Region> = raw_data
        .lines()
        .skip_while(|line| !line.contains("x"))
        .map(Region::from)
        .collect();

    println!("{:?}\n{:?}", shapes, regions);

    let mut regions_able_to_fit_all_presents = 0;
    for region in regions {
        if region.can_fit(&shapes[..]) {
            regions_able_to_fit_all_presents += 1;
        }
    }

    regions_able_to_fit_all_presents
}

#[derive(Debug)]
struct Shape {
    index: u8,
    shape: [[usize; 3]; 3],
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
            shape: [[0, 0, 0], [0, 0, 0], [0, 0, 0]],
        };

        for (row, line) in lines {
            for c in 0..3 {
                s.shape[row - 1][c] = if let Some(ch) = line.chars().nth(c) {
                    if ch == '#' { 1 } else { 0 }
                } else {
                    0
                }
            }
        }

        s
    }
}

impl Shape {
    fn rotated(&self) -> Shape {
        let mut rotated_shape = [[0usize; 3]; 3];

        // rotate 90 degree clockwise
        // [1,2,3]
        // [4,5,6]
        // [7,8,9]
        //   ------>|
        //          v
        // [7,4,1]
        // [8,5,2]
        // [9,6,3]
        // row 0 becomes col 2
        // row 1 becomes col 1
        // row 2 becomes col 0

        rotated_shape[0][2] = self.shape[0][0];
        rotated_shape[1][2] = self.shape[0][1];
        rotated_shape[2][2] = self.shape[0][2];

        rotated_shape[0][1] = self.shape[1][0];
        rotated_shape[1][1] = self.shape[1][1];
        rotated_shape[2][1] = self.shape[1][2];

        rotated_shape[0][0] = self.shape[2][0];
        rotated_shape[1][0] = self.shape[2][1];
        rotated_shape[2][0] = self.shape[2][2];

        Shape {
            index: self.index,
            shape: rotated_shape,
        }
    }
}

#[cfg(test)]
mod test_shapes {
    use super::*;

    #[test]
    fn test_parse() {
        let full: Shape = "0:\n###\n###\n###".into();
        assert_eq!(full.shape, [[1, 1, 1], [1, 1, 1], [1, 1, 1]]);
        assert_eq!(full.index, 0);

        let partial: Shape = "1:\n.##\n#.#\n##.".into();
        assert_eq!(partial.shape, [[0, 1, 1], [1, 0, 1], [1, 1, 0]]);
        assert_eq!(partial.index, 1);

        let none: Shape = "2:\n...\n...\n...".into();
        assert_eq!(none.shape, [[0, 0, 0], [0, 0, 0], [0, 0, 0]]);
        assert_eq!(none.index, 2);
    }

    #[test]
    fn test_rotation() {
        #[rustfmt::skip]
        let shape = Shape {
            index: 0,
            shape: [
                [1, 1, 0], 
                [1, 0, 1], 
                [0, 1, 0],
            ],
        };

        #[rustfmt::skip]
        let rotated = Shape {
            index: 0,
            shape: [
                [0, 1, 1], 
                [1, 0, 1], 
                [0, 1, 0],
            ],
        };
        assert_eq!(rotated.shape, shape.rotated().shape);

        #[rustfmt::skip]
        let rotated = Shape {
            index: 0,
            shape: [
                [0, 1, 0], 
                [1, 0, 1], 
                [0, 1, 1],
            ],
        };
        assert_eq!(rotated.shape, shape.rotated().rotated().shape);

        #[rustfmt::skip]
        let rotated = Shape {
            index: 0,
            shape: [
                [0, 1, 0], 
                [1, 0, 1], 
                [1, 1, 0],
            ],
        };
        assert_eq!(rotated.shape, shape.rotated().rotated().rotated().shape);

        // 4 90 degree turns is an identity function
        assert_eq!(
            shape.shape,
            shape.rotated().rotated().rotated().rotated().shape
        );
    }
}

#[derive(Debug)]
struct Region {
    width: u8,
    height: u8,
    quantity_to_fit_per_shape: [usize; 6],
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
            quantity_to_fit_per_shape: counts,
        }
    }
}

impl Region {
    fn can_fit(&self, regions: &[Shape]) -> bool {
        self.can_fit_n(regions) != 0
    }

    fn can_fit_n(&self, regions: &[Shape]) -> usize {
        // can_fit_n so that we can more easily test the example cases
        0
    }

    fn shapes_for_region<'a>(&self, regions: &'a [Shape]) -> &'a [Shape] {
        regions
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
        assert_eq!(region.quantity_to_fit_per_shape, [38, 37, 45, 42, 54, 41]);

        let region: Region = "4x4: 38 37 45 42 54 41".into();
        assert_eq!(region.width, 4);
        assert_eq!(region.height, 4);
        assert_eq!(region.quantity_to_fit_per_shape, [38, 37, 45, 42, 54, 41]);
    }

    fn example_regions() -> [Region; 3] {
        [
            Region {
                width: 4,
                height: 4,
                quantity_to_fit_per_shape: [0, 0, 0, 0, 2, 0],
            },
            Region {
                width: 12,
                height: 5,
                quantity_to_fit_per_shape: [1, 0, 1, 0, 2, 2],
            },
            Region {
                width: 12,
                height: 5,
                quantity_to_fit_per_shape: [1, 0, 1, 0, 3, 2],
            },
        ]
    }

    fn example_shapes() -> [Shape; 6] {
        [
            Shape {
                index: 0,
                shape: [[1, 1, 1], [1, 1, 0], [1, 1, 0]],
            },
            Shape {
                index: 1,
                shape: [[1, 1, 1], [1, 1, 0], [0, 1, 1]],
            },
            Shape {
                index: 2,
                shape: [[0, 1, 1], [1, 1, 1], [1, 1, 0]],
            },
            Shape {
                index: 3,
                shape: [[1, 1, 0], [1, 1, 1], [1, 1, 0]],
            },
            Shape {
                index: 4,
                shape: [[1, 1, 1], [1, 0, 0], [1, 1, 1]],
            },
            Shape {
                index: 5,
                shape: [[1, 1, 1], [0, 1, 0], [1, 1, 1]],
            },
        ]
    }

    #[test]
    fn test_region_1_example() {
        let region = &example_regions()[0];
        let shapes = example_shapes();
        assert_eq!(region.can_fit_n(&shapes[..]), 2);
    }

    #[test]
    fn test_region_2_example() {
        let region = &example_regions()[1];
        let shapes = example_shapes();
        assert_eq!(region.can_fit_n(&shapes[..]), 5);
    }

    #[test]
    fn test_region_3_example() {
        let region = &example_regions()[2];
        let shapes = example_shapes();
        assert_eq!(region.can_fit_n(&shapes[..]), 0);
    }
}

// TODO: define rotate/flip functions, overlaps?

fn p2(_raw_data: &str) -> ResultType {
    0
}
