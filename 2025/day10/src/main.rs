use std::fs;
use std::time::Instant;
use std::collections::{HashSet, VecDeque};

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
    let configurations: Vec<(Vec<u8>, Vec<Vec<u8>>, Vec<usize>)> = raw_data.lines().map(|line| parse(line)).collect();
    let mut total_presses = 0;
    for configuration in configurations {
        let (goal, buttons, _) = configuration;
        total_presses += fewest_presses(goal, buttons);
    }
    total_presses
}

fn p2(raw_data: &str) -> ResultType {
    let configurations: Vec<(Vec<u8>, Vec<Vec<u8>>, Vec<usize>)> = raw_data.lines().map(|line| parse(line)).collect();
    let mut total_presses = 0;
    for configuration in configurations {
        let (goal, buttons, joltages) = configuration;
        total_presses += fewest_presses_with_joltage(goal, buttons, joltages);
    }
    total_presses
}

fn parse_buttons(btn_part: &str, n: usize) -> Vec<Vec<u8>> {
    let mut result = Vec::new();
    for chunk in btn_part.split(")").filter(|c| !c.trim().is_empty()) {
        // chunk looks like "(0,2,5"
        let inside = chunk.trim_start_matches('(').trim();
        if inside.is_empty() { continue; }

        let mut mask = vec![0u8; n];
        for num in inside.split(',') {
            let idx: usize = num.trim().parse().unwrap();
            mask[idx] = 1;
        }
        result.push(mask);
    }
    result
}
fn parse(line: &str) -> (Vec<u8>, Vec<Vec<u8>>, Vec<usize>) {
    let goal_str = line
        .split_once('[').unwrap().1
        .split_once(']').unwrap().0;

    let goal: Vec<u8> = goal_str
        .chars()
        .filter_map(|c| match c {
            '.' => Some(0),
            '#' => Some(1),
            _ => None,
        })
        .collect();
    let after_goal = line.split_once(']').unwrap().1;

    let mut buttons = Vec::new();
    for group in after_goal.split('(').skip(1) { // skip before first '('
        let inside = group.split_once(')').unwrap().0; // take content inside ()
        if inside.trim().is_empty() { continue; }

        let mut mask = vec![0u8; goal.len()];
        for num in inside.split(',') {
            let idx: usize = num.trim().parse().unwrap();
            mask[idx] = 1;
        }

        buttons.push(mask);
    }

    let joltage_str = line.split_once('{').unwrap().1.split_once('}').unwrap().0;
    let joltages = joltage_str
        .split(',')
        .map(|s| s.trim().parse().unwrap())
        .collect();

    (goal, buttons, joltages)
}

fn fewest_presses_with_joltage(goal: Vec<u8>, buttons: Vec<Vec<u8>>, joltages_goal: Vec<usize>) -> usize {
    // same deal as presses but now our state is bigger.
    // contraint is that joltage only ever INCREASES, so we should be able to leverage that.
    let n = goal.len();
    let start = vec![0u8; n];
    let start_joltage = vec![0usize; n];

    // We just BFS to explore all options brute force for now
    let mut q = VecDeque::new();
    q.push_back(((start.clone(), start_joltage.clone()), 0usize));
    let mut visited = HashSet::new();
    visited.insert((start, start_joltage));

    while let Some(((state, joltage), dist)) = q.pop_front() {

        if machine_done(&state, &goal) && joltage_matches(&joltage, &joltages_goal) {
            return dist;
        }

        for btn in &buttons {
            let next_state = apply_button(state.clone(), btn);
            let next_jolt = apply_joltage(joltage.clone(), btn);
            println!("dist={} state={:?} jolt={:?}", dist, next_state, next_jolt);

            if !joltage_invalid(&next_jolt, &joltages_goal) {
                let key = (next_state.clone(), next_jolt.clone());
                if visited.insert(key) {
                    q.push_back(((next_state, next_jolt), dist + 1));
                }
            }
        }
    }

    // arbitrary nonsense.
    println!("{:?} {:?} JoltGoal: {:?}", goal, buttons, joltages_goal);
    usize::MAX
}

fn fewest_presses(goal: Vec<u8>, buttons: Vec<Vec<u8>>) -> usize {
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
            let next = apply_button(state.clone(), btn);

            if !visited.contains(&next) {
                if machine_done(&next, &goal) {
                    return dist + 1;
                }

                visited.insert(next.clone());
                q.push_back((next, dist + 1));
            }
        }
    }

    // arbitrary nonsense.
    println!("{:?} {:?}", goal, buttons);
    usize::MAX
}

fn apply_button(mut machine: Vec<u8>, button_click: &[u8]) -> Vec<u8> {
    for i in 0..machine.len() {
        machine[i] = machine[i] ^ button_click[i];
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

fn apply_joltage(mut joltage: Vec<usize>, button_click: &[u8]) -> Vec<usize> {
    for i in 0..joltage.len() {
        if button_click[i] == 1 {
            joltage[i] += 1;
        }
    }
    joltage
}

fn joltage_matches(joltage: &Vec<usize>, goal: &Vec<usize>) -> bool {
    for i in 0..joltage.len() {
        if joltage[i] != goal[i] {
            return false
        }
    }
    true
}

fn joltage_invalid(joltage: &Vec<usize>, goal: &Vec<usize>) -> bool {
    for i in 0..joltage.len() {
        if joltage[i] > goal[i] {
            return true;
        }
    }
    return false;
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

    #[test]
    fn test_breaking_machine_1() {
        let (m, buttons, _) = parse("[##..] (1,3) (1) (1,2) (0,1) (2,3) {13,66,31,27}");
        assert_eq!(1, fewest_presses(m, buttons));
    }

    #[test]
    fn test_joltage_apply() {
        let joltage = vec![1,0,0,1];
        let joltage =  apply_joltage(joltage, &[0, 1, 1, 0]);
        assert_eq!(joltage, [1, 1, 1, 1])
    }

    #[test]
    fn test_joltage_match() {
        let joltage = vec![1,0,0,1];
        let jolt_goal = vec![1,0,0,1];
        assert!(joltage_matches(&joltage, &jolt_goal));
    }

    #[test]
    fn test_joltage_invalid_when_bigger() {
        let joltage = vec![2,0,0,1];
        let jolt_goal = vec![1,0,0,1];
        assert!(joltage_invalid(&joltage, &jolt_goal));
    }

    #[test]
    fn test_first_machine_with_joltage() {
        let (m, buttons, joltage_goal) = parse("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}");
        assert_eq!(10, fewest_presses_with_joltage(m, buttons, joltage_goal));
    }

    #[test]
    fn test_second_machine_with_joltage() {
        let (m, buttons, joltage_goal) = parse("[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}");
        assert_eq!(12, fewest_presses_with_joltage(m, buttons, joltage_goal));
    }

    #[test]
    fn test_third_machine_with_joltage() {
        let (m, buttons, joltage_goal) = parse("[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}");
        assert_eq!(11, fewest_presses_with_joltage(m, buttons, joltage_goal));
    }

}
