use std::fs;
use std::time::Instant;
use std::collections::HashSet;
use std::cmp::{min, max};

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
    let tiles: Vec<(ResultType, ResultType)> = raw_data.lines().take_while(|line| !line.is_empty()).map(|line| {
        let mut xy = line.split(",");
        (
            xy.next().expect("no digit x").parse().expect("bad number x"),
            xy.next().expect("no digit y").parse().expect("bad number y")
        )
    }).collect();
    
    let mut areas = vec![];
    for p1 in tiles.iter() {
        for p2 in tiles.iter() {
            if p1 == p2 {
                continue;
            }
            let area = (1 + (p1.0 - p2.0).abs()) * (1 + (p1.1 - p2.1).abs());
            areas.push(area);
        }
    }
    areas.sort();
    *areas.iter().rev().take(1).next().expect("No answer")
}

fn p2(raw_data: &str) -> ResultType {
    let red_tiles: Vec<(ResultType, ResultType)> = raw_data.lines().take_while(|line| !line.is_empty()).map(|line| {
        let mut xy = line.split(",");
        (
            xy.next().expect("no digit x").parse().expect("bad number x"),
            xy.next().expect("no digit y").parse().expect("bad number y")
        )
    }).collect();

    let mut max_area = 0;
    for p1 in &red_tiles {
        for  p2 in &red_tiles {
            if p1 == p2 {
                continue;
            }

            let left   = min(p1.0, p2.0);
            let right  = max(p1.0, p2.0);
            let top    = min(p1.1, p2.1);
            let bottom = max(p1.1, p2.1);

            // are the corners inside of the polygon?
            if !(
                point_in_poly(left, top, &red_tiles) &&
                point_in_poly(left, bottom, &red_tiles) &&
                point_in_poly(right, top, &red_tiles) &&
                point_in_poly(right, bottom, &red_tiles)
            ) {
                continue;
            }


            // 
            let mut blocked = false;
            for i in 0..red_tiles.len() {
                let a = red_tiles[i];
                let b = red_tiles[(i+1) % red_tiles.len()];
                if adjacent_edge_intersects(( (left, top), (right, bottom) ), (a,b)) {
                    blocked = true;
                    break;
                }
            }
            if blocked { 
                continue; 
            }

            let area = (1 + (p1.0 - p2.0).abs()) * (1 + (p1.1 - p2.1).abs());
            if area > max_area {
                max_area = area;
            }
        }
    }

    max_area
}


/// Returns true if (x,y) is inside the polygon using the even–odd rule.
/// 
/// Search terms: "point in polygon even odd", "ray casting algorithm"
pub fn point_in_poly(x: i64, y: i64, poly: &[(i64,i64)]) -> bool {
    unimplemented!()
}

/// Given a rectangle defined by two opposite corners rect1 and rect2,
/// and an adjacent polygon edge (p1,p2), returns true if the polygon
/// edge crosses into the *interior* of the rectangle.
///
/// Search terms: 
/// "line segment intersection with bounding box"
/// "range overlap check"
pub fn adjacent_edge_intersects(
    rect: ((i64,i64),(i64,i64)),
    edge: ((i64,i64),(i64,i64)),
) -> bool {
    unimplemented!()
}

/// Checks if the axis-aligned rectangle between p1 and p2 is valid:
/// - All four corners are inside the polygon (even–odd rule)
/// - No polygon edge intersects the interior of the rectangle
///
/// Search terms:
/// "axis aligned rectangle interior test"
/// "polygon edge vs bounding box intersection"
pub fn is_rectangle_valid(
    p1: (i64,i64),
    p2: (i64,i64),
    poly: &[(i64,i64)],
    adjacency_edges: &[((i64,i64),(i64,i64))],
) -> bool {
    unimplemented!()
}

/// Computes the geometric area of the rectangle defined by p1 and p2.
/// NOTE: NO +1 terms. This puzzle uses dx * dy.
///
/// Search terms:
/// "geometry compute rectangular area from two points"
pub fn rectangle_area(p1: (i64,i64), p2: (i64,i64)) -> i64 {
    unimplemented!()
}

/// Iterates over all pairs of red tiles and returns the largest valid
/// rectangle area.
///
/// Search terms:
/// "pairwise iteration", "combinatorics n^2 pair generation",
/// "filter + max element"
pub fn largest_valid_rectangle(poly: &[(i64,i64)]) -> i64 {
    unimplemented!()
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

    #[test]
    fn test_point_in_polygon_complex_shape() {
        let poly = vec![
            (7,1),(11,1),(11,7),(9,7),(9,5),(2,5),(2,3),(7,3)
        ];

        assert!(point_in_poly(8,4,&poly));  // inside
        assert!(!point_in_poly(1,1,&poly)); // outside
    }

    #[test]
    fn test_adj_edge_does_not_intersect_valid_rect() {
        let rect = ((2,3),(9,5));
        let edge = ((7,1),(11,1)); // horizontal above rectangle

        assert!(!adjacent_edge_intersects(rect, edge));
    }

    #[test]
    fn test_adj_edge_intersects_invalid_rect() {
        let rect = ((2,3),(9,5));
        let edge = ((9,5),(9,7)); // vertical edge crosses right side

        assert!(adjacent_edge_intersects(rect, edge));
    }

    #[test]
    fn test_rectangle_area() {
        let p1 = (2,3);
        let p2 = (9,5);

        assert_eq!(rectangle_area(p1, p2), 14); // dx=7, dy=2 → 14
    }

    #[test]
    fn test_puzzle_example_biggest_area_24() {
        let poly = vec![
            (7,1),(11,1),(11,7),(9,7),(9,5),(2,5),(2,3),(7,3)
        ];

        let area = largest_valid_rectangle(&poly);

        assert_eq!(area, 24);
    }
}
