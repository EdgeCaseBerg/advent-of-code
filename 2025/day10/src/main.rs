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
fn p1(raw_data: &str) -> ResultType {
    let configurations: Vec<(Vec<u8>, Vec<Vec<usize>>, Vec<usize>)> = raw_data.lines().map(|line| parse(line)).collect();
    let mut total_presses = 0;
    for configuration in configurations {
        let (goal, buttons, _) = configuration;
        total_presses += fewest_presses(goal, buttons);
    }
    total_presses
}

fn p2(_raw_data: &str) -> ResultType {
    0
}

fn parse(line: &str) -> (Vec<u8>, Vec<Vec<usize>>, Vec<usize>) {
    let (goal_part, rest) = line.split_once(" ").expect("couldnt split");
    let goal: Vec<u8> = goal_part.chars().fold(vec![], |mut acc, c| {
        match c {
            '.' => { acc.push(0); acc },
            '#' => { acc.push(1); acc },
            _ => acc,
        }
    });

    let (btn_part, rest) = rest.split_once("{").expect("couldnt split");
    let mut buttons: Vec<Vec<usize>> = vec![];
    for raw in btn_part.split(" ").filter(|x| !x.trim().is_empty()) {
        let mut digits = vec![0; goal.len()];
        let mut b = String::new();
        for c in raw.chars() {
            match c {
                '0'..'9' => {
                    b.push(c);
                }
                ',' | ')' => {
                    let idx: usize = b.parse().expect("could not parse number for button");
                    digits[idx] = 1;
                    b = String::new();
                },
                _ => {}

            }
        }
        buttons.push(digits.to_vec());
    }
    let mut joltage = vec![];
    let mut s = String::new();
    for c in rest.chars() {
        match c {
            '0'..'9' => {
                s.push(c);
            },
            _ => {
                joltage.push(s.parse().expect("cant parse joltage"));
                s = String::new();
            },
        }
    }

    (goal, buttons, joltage)
}

// fn fewest_presses(goal: Vec<u8>, buttons: Vec<Vec<usize>>) -> usize {
//     // Just BFS it for now.
//     // initial state -> 
//     //    b1 -> b2 ...
//     //    b2 -> b3 ...
//     // etc. Given b1 + b1 = b1, we can safely ignore pressing the same button as the current depth
//     let mut machine = vec![0; goal.len()];
//     let mut i = 0;
//     for btn in buttons {
//         machine = apply_button(machine, btn);
//         if machine_done(&machine, &goal) {
//             return i;
//         }
//         println!("{:?}", machine);
//     }
//     i
// }

use std::collections::{HashSet, VecDeque};

fn fewest_presses(goal: Vec<u8>, buttons: Vec<Vec<usize>>) -> usize {
    let n = goal.len();
    let start = vec![0u8; n];

    if start == goal {
        return 0;
    }

    // We just BFS to explore all options brute force for now
    let mut q = VecDeque::new();
    q.push_back((start.clone(), 0usize));
    let mut visited = HashSet::new();
    visited.insert(start);

    while let Some((state, dist)) = q.pop_front() {
        for btn in &buttons {
            let next = apply_button(state.clone(), btn.clone());

            if !visited.contains(&next) {
                if next == goal {
                    return dist + 1;
                }

                visited.insert(next.clone());
                q.push_back((next, dist + 1));
            }
        }
    }

    // arbitrary nonsense.
    1000000000
}

fn apply_button(mut machine: Vec<u8>, button_click: Vec<usize>) -> Vec<u8> {
    for i in 0..machine.len() {
        machine[i] = machine[i] ^ button_click[i] as u8;
    }
    machine
}

fn machine_done(machine: &Vec<u8>, goal: &Vec<u8>) -> bool {
    for i in 0..machine.len() {
        if machine[i] != goal[i] {
            return false
        }
    }
    true
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let (m, buttons, joltages) = parse("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}");

        assert_eq!(m, vec![0,1,1,0]);
        assert_eq!(buttons, vec![
            vec![0,0,0,1],
            vec![0,1,0,1],
            vec![0,0,1,0],
            vec![0,0,1,1],
            vec![1,0,1,0],
            vec![1,1,0,0]
        ]);
        assert_eq!(joltages, vec![3,5,4,7])
    }

    #[test]
    fn test_first_machine() {
        let (m, buttons, _) = parse("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}");
        assert_eq!(2, fewest_presses(m, buttons));
    }

    #[test]
    fn test_second_machine() {
        let (m, buttons, _) = parse("[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}");
        assert_eq!(3, fewest_presses(m, buttons));
    }

    #[test]
    fn test_third_machine() {
        let (m, buttons, _) = parse("[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}");
        assert_eq!(2, fewest_presses(m, buttons));
    }

}
