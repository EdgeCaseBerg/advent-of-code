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

type ResultType = usize;
fn p1(_raw_data: &str) -> ResultType {
    0
}

fn p2(_raw_data: &str) -> ResultType {
    0
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_in_polygon_basic() {
        let poly = vec![(0,0), (10,0), (10,10), (0,10)];

        assert!(point_in_poly(5, 5, &poly));   // inside
        assert!(!point_in_poly(-1, 5, &poly)); // outside left
        assert!(!point_in_poly(5, 11, &poly)); // outside top
    }

}
