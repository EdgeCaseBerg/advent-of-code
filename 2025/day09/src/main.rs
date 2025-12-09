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
    largest_valid_rectangle(&red_tiles)
}


/// Returns true if (x,y) is inside the polygon using the even–odd rule.
/// The given x y is the scanline
/// similar to stuff like https://wrfranklin.org/Research/Short_Notes/pnpoly.html
pub fn point_in_poly(x: ResultType, y: ResultType, poly: &[(ResultType,ResultType)]) -> bool {
    let mut inside = false;
    let mut j = poly.len() - 1;

    for i in 0..poly.len() {
        let (ax, ay) = poly[i];
        let (bx, by) = poly[j];

        // Check if the test point is on a vertex
        if x == ax && y == ay {
            return true;
        }

        // Edge crosses scanline?
        let intersects = (ay > y) != (by > y)
            && x < (bx - ax) * (y - ay) / (by - ay) + ax;

        if intersects {
            inside = !inside;
        }

        j = i;
    }

    inside
}

/// Given a rectangle defined by two opposite corners rect1 and rect2,
/// and an adjacent polygon edge (p1,p2), returns true if the polygon
/// edge crosses into the *interior* of the rectangle.
///
/// https://stackoverflow.com/questions/3235385/given-a-bounding-box-and-a-line-two-points-determine-if-the-line-intersects-t
pub fn adjacent_edge_intersects(
    rect: ((ResultType,ResultType),(ResultType,ResultType)),
    edge: ((ResultType,ResultType),(ResultType,ResultType)),
) -> bool {
    let ((x1, y1), (x2, y2)) = rect;

    let left   = x1.min(x2);
    let right  = x1.max(x2);
    let top    = y1.min(y2);
    let bottom = y1.max(y2);

    let ((ex1, ey1), (ex2, ey2)) = edge;

    // edge is vertical because x is same.
    if ex1 == ex2 {
        let x_between_points = left < ex1 && ex1 < right;
        let edge_top = ey1.min(ey2);
        let edge_bottom = ey1.max(ey2);
        if x_between_points {
            return !(edge_bottom <= top || edge_top >= bottom);
        }
        return false;
    }

    // edge is horizontal because y is same.
    if ey1 == ey2 {
        // 0 is at the top of the "grid" in this case. smaller is "higher"
        let y_between_points = top < ey1 && ey1 < bottom;
        let edge_left = ex1.min(ex2);
        let edge_right = ex1.max(ex2);
        if y_between_points {
            return !(edge_right <= left || edge_left >= right);
        }
        return false;
    }

    false
}

/// plus 1 to deal with the grid nature of things. a single line still has area of 1
pub fn rectangle_area(p1: (ResultType,ResultType), p2: (ResultType,ResultType)) -> ResultType {
    (1 + (p1.0 - p2.0).abs()) * (1 + (p1.1 - p2.1).abs())
}

pub fn largest_valid_rectangle(red_tiles: &[(ResultType,ResultType)]) -> ResultType {
    let mut max_area = 0;
    for p1 in red_tiles {
        for  p2 in red_tiles {
            if p1 == p2 {
                continue;
            }

            let left   = min(p1.0, p2.0);
            let right  = max(p1.0, p2.0);
            let top    = min(p1.1, p2.1);
            let bottom = max(p1.1, p2.1);

            // are the corners inside of the polygon?
            if !(
                point_in_poly(left, top, red_tiles) &&
                point_in_poly(left, bottom, red_tiles) &&
                point_in_poly(right, top, red_tiles) &&
                point_in_poly(right, bottom, red_tiles)
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
        let edge = ((8,2),(8,6)); 
        // vertical line at x=8, spanning y=2..6 → passes through interior

        assert!(adjacent_edge_intersects(rect, edge));
    }

    #[test]
    fn edge_horizontal_crosses_rect_interior() {
        let rect = ((2, 3), (9, 5));
        let edge = ((0, 4), (20, 4)); // passes straight through rectangle interior
        assert!(adjacent_edge_intersects(rect, edge));
    }

    #[test]
    fn edge_horizontal_outside_rect_does_not_intersect() {
        let rect = ((2, 3), (9, 5));
        let edge = ((0, 2), (20, 2)); // entirely below
        assert!(!adjacent_edge_intersects(rect, edge));
    }

    #[test]
    fn edge_horizontal_touching_top_boundary_does_not_intersect() {
        let rect = ((2, 3), (9, 5));
        let edge = ((0, 3), (20, 3)); // exactly on top boundary
        assert!(!adjacent_edge_intersects(rect, edge));
    }

    #[test]
    fn edge_horizontal_touching_bottom_boundary_does_not_intersect() {
        let rect = ((2, 3), (9, 5));
        let edge = ((0, 5), (20, 5)); // exactly on bottom boundary
        assert!(!adjacent_edge_intersects(rect, edge));
    }

    #[test]
    fn edge_vertical_crosses_rect_interior() {
        let rect = ((2, 3), (9, 5));
        let edge = ((4, 0), (4, 20)); // passes straight through rectangle interior
        assert!(adjacent_edge_intersects(rect, edge));
    }

    #[test]
    fn edge_vertical_outside_rect_does_not_intersect() {
        let rect = ((2, 3), (9, 5));
        let edge = ((10, 0), (10, 20)); // entirely to the right
        assert!(!adjacent_edge_intersects(rect, edge));
    }

    #[test]
    fn edge_vertical_touching_left_boundary_does_not_intersect() {
        let rect = ((2, 3), (9, 5));
        let edge = ((2, 0), (2, 20)); // exactly on left boundary
        assert!(!adjacent_edge_intersects(rect, edge));
    }

    #[test]
    fn edge_vertical_touching_right_boundary_does_not_intersect() {
        let rect = ((2, 3), (9, 5));
        let edge = ((9, 0), (9, 20)); // exactly on right boundary
        assert!(!adjacent_edge_intersects(rect, edge));
    }


    #[test]
    fn test_rectangle_area() {
        let p1 = (2,3);
        let p2 = (9,5);

        assert_eq!(rectangle_area(p1, p2), 24);
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
