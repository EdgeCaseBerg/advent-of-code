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
        let (_, buttons, joltages) = configuration;
        total_presses += fewest_presses_with_joltage(buttons, joltages);
    }
    total_presses
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




fn fewest_presses_with_joltage(buttons: Vec<Vec<u8>>, jolt_goal: Vec<usize>) -> usize {
    /* The input comes in like this:
     * [ [1, 0], [0,1] ]      [1, 1]
     * and then the goal is to know we should press the buttons once each to make 
     * the initial state of [0,0] become [1, 1]. So, this is sorta like this:
     * 
     *  1a 0b = 1 jolt
     *  0a 0b = 1 jolt
     *
     * Which is REALLY similar to matrices in linear algebra.
     * since you can never DECREASE the joltage, it only EVER goes up, then this is likke
     *
     * 1a + 0b = 1
     * 0a + 1b = 1
     *
     * And for a more complicated example, 
     * [ [1, 0 ], [1, 1] ]   [ 3, 2]
     *
     * a = 3
     * a + b = 2
     *
     * which of course trivially rduces down to press b1 once, b2 twice but... what about the case
     * of:
     * [ [1, 0 ], [1, 1], [ 0, 1]]   [ 3, 2]
     *
     * this has MORE than 1 way to get to 3,2 and if we want to hit on the min b2 + b1, and not land
     * on b1 + b1 + b2 + b3 or some other combination, then we need to figure out how to get the smallest
     * one of these.
     *
     * We can never do a negative button press, so if we were to solve an question then we can toss out
     * any unbounded variable that needs to get loved by someone who takes a more negative viewpoint of the world.
     * 
     * But first... convert the button input into a matrix since matrices are a good way to deal with 
     * computing equations Linear algebra baby! one row per joltage output!
     */
    let height = jolt_goal.len();
    let width = 1 + buttons.len(); // 1 + because it's a + b = c and we need space for c in the matrix.
    let mut matrix: Vec<Vec<i32>> = Vec::new();
    for _ in 0..height {
        let row = vec![0i32; width];
        matrix.push(row);
    }

    for (r, coefficient) in jolt_goal.into_iter().enumerate() {
        matrix[r][width - 1] = coefficient as i32;
    }

    for b in 0..buttons.len() {
        let button = &buttons[b];
        for (c, affects) in button.iter().enumerate() {
            if *affects == 1  {
                matrix[c][b] = 1;
            }
        }
    }

    // we'll get back how many presses of each button needs to happen
    // so the total presses is just heir sum
    let presses = gauss_it_up(matrix, buttons.len());
    presses.iter().sum()
}

fn gauss_it_up(matrix: Vec<Vec<i32>>, variable_count: usize) -> Vec<usize> {
    /* Matrices are hard. I kind of feel like grabbing a library to do this sort
     * of thing would PROBABLY be the better option here. But eh. Let's see if we
     * can figure it out.
     * 
     * First off, our matrix isn't in the normalized form we need to do solving.
     * So, first we need to put it into that!
     */
    let rows = matrix.len();
    let columns = matrix[0].len();

    let mut normalized_matrix: Vec<Vec<i32>> = Vec::new();
    for r in 0..rows {
        normalized_matrix.push(Vec::new());
        for c in 0..columns {
            normalized_matrix[r].push(matrix[r][c]);
        }
    }

    /* We need to find the pivot of the matrix */
    let mut pivot_row = 0;
    let mut pivol_column = vec![usize::MAX; rows];
    let mut is_pivot_column_tracker = vec![false; variable_count];

    let mut col = 0;
    loop {
        /* I dont remember how to for (x=y;condition;++) in rust */
        if col >= variable_count && pivot_row >= rows {
            break;
        }

        let mut row = pivot_row;
        let mut best_row = usize::MAX;

        loop {
            if row >= rows {
                break;
            }
            if normalized_matrix[row][col] != 0 {
                if best_row == usize::MAX || normalized_matrix[row][col].abs() == 1 {
                    best_row = row;
                    if normalized_matrix[row][col].abs() == 1 {
                        break;
                    }
                }
            }

            row += 1;
        }

        /* No best row found, so check the next one. */
        if best_row == usize::MAX {
            col += 1;
            continue;
        }

        /* Rust has a swap! no clone() or anything :)  */
        normalized_matrix.swap(best_row, pivot_row);

        /* Track the pivot, and then... */
        pivol_column[pivot_row] = col;
        is_pivot_column_tracker[col] = true;

        /* We can now elimate this column in other rows */
        for r in 0..rows {
            if row != pivot_row && normalized_matrix[r][col] != 0 {
                let factor = normalized_matrix[r][col];
                for c in 0..columns {
                    normalized_matrix[r][c] -= factor * normalized_matrix[pivot_row][c];
                }
            }
        }


        /* Don't forget to bump the "for loop" variables up and also shift the pivot row to the next row */
        pivot_row += 1;
        col += 1;
        break;
    }

    /* Assume that all inputs have a solution, so don't bother checking consistency 
     * if we DO need to that, then check that each number is greater than 0.
     * So now, we need to know about the variables that are floating around that we'll
     * need to try to determine.
     */
    let mut free_variables = vec![];
    for c in 0..variable_count {
        if !is_pivot_column_tracker[c] {
            free_variables.push(c);
        }
    }

    /* If every variable is already bound, then we can check to see if we can return early
     * without having to do any extra work:
     */
    if free_variables.is_empty() {
        let mut solution = vec![0usize; variable_count];
        for r in 0..pivot_row {
            let col = pivol_column[r];
            let value = normalized_matrix[r][columns - 1];
            if value < 0 || (value - normalized_matrix[r][columns - 1]).abs() > 0 {
                println!("No solution it seems?");
                return Vec::new();
            }
            solution[col] = value as usize;
        }
        return solution;
    }

    /* But if we DO have a system of equations that has free variables, then we need to 
     * search in the space of the potential values for those for something that works
     * but since numbers are infinity, we'll constrain it arbitrarily based on if the AoC
     * puzzle tells us if the number is too low or high.
     * We can smartly constrain the maximum number of presses down to the sum of all the
     * potential joltage targets since if we go past that the solution is garbage:
    */
    let maybe_solution = solve_for_free_variables(free_variables, &normalized_matrix, &pivol_column, pivot_row, variable_count);

    maybe_solution
}

fn solve_for_free_variables(
    free_vars: Vec<usize>,
    normalized: &Vec<Vec<i32>>,
    pivol_column: &Vec<usize>,
    pivot_row: usize,
    variable_count: usize,
) -> Vec<usize> {

    // let max_target = matrix.iter().map(|row| row[columns - 1]).max();
    // let max_free_variable_value = max_target.min(500) // 100 was too little, 500 seems ok.

    // let mut min_solution = Vec::new();
    // let mut min_sum_found = i32:MAX;

    Vec::new()
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
        let (_, buttons, joltage_goal) = parse("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}");
        assert_eq!(10, fewest_presses_with_joltage(buttons, joltage_goal));
    }

    #[test]
    fn test_second_machine_with_joltage() {
        let (_, buttons, joltage_goal) = parse("[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}");
        assert_eq!(12, fewest_presses_with_joltage(buttons, joltage_goal));
    }

    #[test]
    fn test_third_machine_with_joltage() {
        let (_, buttons, joltage_goal) = parse("[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}");
        assert_eq!(11, fewest_presses_with_joltage(buttons, joltage_goal));
    }

}
