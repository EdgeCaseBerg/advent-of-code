use std::fs;
use std::time::Instant;
use std::collections::HashSet;

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

type ResultType = i128;

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
    println!("{:?}", "going red...");


    // Pre-compute the bounding box around the grid so that we can easily tell what is
    // "outside" versus "inside" so that we can start inside and flood in the right
    // direction.
    let mut min_x = ResultType::MAX;
    let mut max_x = ResultType::MIN;
    let mut min_y = ResultType::MAX;
    let mut max_y = ResultType::MIN;

    // println!("{:?}", "going green...");
    // let mut green_tiles = HashSet::new();
    // let n = red_tiles.len();
    // // Time to draw straight lines of green between red tiles.
    // for i in 0..n {
    //     let first = red_tiles[i];

    //     let second = if i + 1 != n {
    //         red_tiles[i + 1]
    //     } else {
    //         red_tiles[0]
    //     };

    //     // there is a green STRAIGHT line between these two points.
    //     let bx = first.0.min(second.0);
    //     let bxMax = first.0.max(second.0);
    //     let by = first.1.min(second.1);
    //     let byMax = first.1.max(second.1);
    //     for x in bx..=bxMax {
    //         for y in by..=byMax {
    //             // we technically count the red tiles as green, but that doesn't matter.
    //             // its basically just a list of valid places to be
    //             green_tiles.insert((x, y));
    //         }
    //     }

    //     // running precompute.
    //     if min_x > bx {
    //         min_x = bx;
    //     }
    //     if max_x < bxMax {
    //         max_x = bxMax;
    //     }
    //     if min_y > by {
    //         min_y = by;
    //     }
    //     if max_y < byMax {
    //         max_y = byMax;
    //     }
    // }
    // But we're not done yet. We have completed the shape, and thus
    // we need to now fill the shape inside of it. This is the actual
    // hard part I suppose.
    // maybe https://en.wikipedia.org/wiki/Point_in_polygon ?
    // let mut start_point = red_tiles[0].clone();
    // let mut crossed = 0;
    // // cast a "ray" from 0,0 to the point
    // for x in 0..=max_x {
    //     for y in 0..=max_y {
    //         if green_tiles.contains(&(x,y)) {
    //             crossed += 1;
    //             if (crossed % 2 == 0) {
    //                 start_point = (x,y - 1);
    //             }
    //         }
    //     }
    // }
    // just choose one based roughtly on the loop connection and guesswork
    // let start_point = (97713,51514);
    // println!("Seeding at {:?}", start_point);
    // let mut queue = Vec::new();
    // queue.push(start_point);
    // while let Some(tile) = queue.pop() {
    //     for dx in -1..=1 {
    //         for dy in -1..=1 {
    //             let new_tile = (tile.0 + dx, tile.1 + dy);
    //             if new_tile.0 < min_x || new_tile.0 > max_x {
    //                 println!("bad seed");
    //                 continue;
    //             }
    //             if new_tile.1 < min_y || new_tile.1 > max_y {
    //                 println!("bad seed");
    //                 continue;
    //             }
    //             if green_tiles.contains(&new_tile) {
    //                 continue;
    //             } else {
    //                 green_tiles.insert(new_tile.clone());
    //                 queue.push(new_tile.clone());
    //                 // println!("{:?}", new_tile);
    //             }
    //         }
    //     }
    // }



    let mut areas = vec![];
    for p1 in red_tiles.iter() {
        for p2 in red_tiles.iter() {
            if p1 == p2 {
                continue;
            }
            let width = 1 + (p1.0 - p2.0).abs();
            let height = 1 + (p1.1 - p2.1).abs();
            let area = width * height;
            let mut inside = 0; 

            let bx = p1.0.min(p2.0);
            let bxMax = p1.0.max(p2.0);
            let by = p1.1.min(p2.1);
            let byMax = p1.1.max(p2.1);
            let mid_x = (bx + bxMax) / 2;
            let mid_y = (by + byMax) / 2;

            let valid = point_in_poly(mid_x, mid_y, &red_tiles)
            && point_in_poly(bx, by, &red_tiles)
            && point_in_poly(bx, byMax, &red_tiles)
            && point_in_poly(bxMax, by, &red_tiles)
            && point_in_poly(bxMax, byMax, &red_tiles);

            
            if valid {
                areas.push((area, p1, p2));
            }
        }
    }
    areas.sort_by_key(|(a, _, _)| *a);

    // Walk backward from the largest area to the smallest
    let mut largest_area = 0;
    for (area, p1, p2) in areas.iter().rev() {
        // let mut is_green = true;
        // let bx = p1.0.min(p2.0);
        // let bxMax = p1.0.max(p2.0);
        // let by = p1.1.min(p2.1);
        // let byMax = p1.1.max(p2.1);
        // for x in bx..=bxMax {
        //     for y in by..=byMax {
        //         // we technically count the red tiles as green, but that doesn't matter.
        //         if !green_tiles.contains(&(x, y)) {
        //             is_green = false;
        //             break;
        //         }
        //     }
        //     if !is_green {
        //         break;
        //     }
        // }
        // println!("{:?} {:?} {:?} {:?} {:?}", is_green, area, largest_area, p1, p2);
        if *area > largest_area {
            largest_area = *area;
        }
    }

    // 4492472302 answer too high
    largest_area
}

fn point_in_poly(x: ResultType, y: ResultType, poly: &Vec<(ResultType, ResultType)>) -> bool {
    let mut crossings = 0;
    let n = poly.len();

    for i in 0..n {
        let (x1, y1) = poly[i];
        let (x2, y2) = poly[(i+1) % n];

        // Check if ray crosses the edge
        let intersects = ((y1 > y) != (y2 > y)) &&
                         (x < (x2 - x1) * (y - y1) / (y2 - y1) + x1);

        if intersects {
            crossings += 1;
        }
    }

    crossings % 2 == 1
}

