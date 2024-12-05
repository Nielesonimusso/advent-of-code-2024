use std::iter::zip;

use advent_of_code_2024::input::get_lines;

fn main() {
    let mut lists: [Vec<i32>; 2] = Default::default();
    for line in get_lines(1) {
        for (index, word) in line.expect("Not a line").split_whitespace().enumerate() {
            lists[index].push(word.parse().expect("Not an i32"));
        }
    }

    for list in &mut lists {
        list.sort();
    }

    assert_eq!(lists[0].len(), lists[1].len());

    let mut sum: u32 = 0;
    for (left, right) in zip(&lists[0], &lists[1]) {
        sum += left.abs_diff(right.clone());
    }

    println!("Result 1a: {}", sum);
}
