use std::collections::HashMap;

use advent_of_code_2024::input::get_lines;

fn main() {
    let mut lists: [Vec<i32>; 2] = Default::default();
    for line in get_lines(1) {
        for (index, word) in line.expect("Not a line").split_whitespace().enumerate() {
            lists[index].push(word.parse().expect("Not an i32"));
        }
    }

    let mut right_counts: HashMap<i32, u32> = HashMap::new();

    for right in &lists[1] {
        right_counts
            .entry(*right)
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }

    println!("amount of keys: {}", right_counts.len());

    let mut sum: u32 = 0;
    for left in &lists[0] {
        sum += (left.clone() as u32) * right_counts.get(left).or(Some(&0)).expect("impossible");
    }

    println!("Result 1b: {}", sum);
}
