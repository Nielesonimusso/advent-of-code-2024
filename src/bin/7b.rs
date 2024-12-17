use advent_of_code_2024::input::get_lines;

use itertools::Itertools;
use regex::Regex;

fn check_operators(
    target: u128,
    accumulator: u128,
    remaining: &[u128],
    used_ops: Vec<char>,
) -> Option<Vec<char>> {
    if accumulator == target && remaining.is_empty() {
        return Some(used_ops);
    } else if !remaining.is_empty() {
        let next_remaining = &remaining[1..];
        let mut used_ops_plus = used_ops.clone();
        used_ops_plus.push('+');
        let mut used_ops_mult = used_ops.clone();
        used_ops_mult.push('*');
        let mut used_ops_concat = used_ops.clone();
        used_ops_concat.push('|');
        return check_operators(
            target,
            accumulator + remaining[0],
            next_remaining,
            used_ops_plus,
        )
        .or(check_operators(
            target,
            accumulator * remaining[0],
            next_remaining,
            used_ops_mult,
        ))
        .or(check_operators(
            target,
            format!("{}{}", accumulator, remaining[0])
                .parse()
                .expect("invalid concat"),
            next_remaining,
            used_ops_concat,
        ));
    } else {
        return None;
    }
}

fn apply_answer(inputs: &[u128], ops: &[char]) -> u128 {
    assert_eq!(inputs.len(), ops.len() + 1);
    inputs[1..]
        .iter()
        .zip(ops.iter())
        .fold(inputs[0], |accum, (input, op)| match *op {
            '+' => accum + input,
            '*' => accum * input,
            '|' => format!("{}{}", accum, input)
                .parse()
                .expect("error concat during apply"),
            _ => panic!("unknown operator during apply..!"),
        })
}

fn main() {
    let line_regex = Regex::new(r"(?<target>\d+):(?<inputs>(?:\s+\d+)+)").expect("wrong regex!");

    let mut sum_possible: u128 = 0;
    let mut num_possible: usize = 0;
    let mut num_impossible: usize = 0;
    let mut lines = get_lines(7);
    while let Some(Ok(line)) = lines.next() {
        if let Some(captures) = line_regex.captures(&line) {
            let target: u128 = captures
                .name("target")
                .and_then(|mat| mat.as_str().parse().ok())
                .expect("no target..!");
            let inputs: Vec<u128> = captures
                .name("inputs")
                .and_then(|mat| Some(mat.as_str().split_ascii_whitespace()))
                .and_then(|split| {
                    Some(
                        split
                            .map(|slice| slice.parse().expect("incorrect input..!"))
                            .collect(),
                    )
                })
                .expect("no input..!");
            if let Some(ops) = check_operators(target, inputs[0], &inputs[1..], Vec::new()) {
                let result = inputs
                    .iter()
                    .map(|input| input.to_string())
                    .interleave(ops.iter().map(|op| op.to_string()))
                    .join(" ");
                print!("P!: {target} := {result}");
                let applied_result = apply_answer(&inputs[..], &ops[..]);
                println!(" (== {applied_result})");
                sum_possible += target;
                num_possible += 1;
            } else {
                num_impossible += 1;
                println!("NP: {target} x= {:?}", inputs);
            }
        }
    }

    println!("Sum of possibles: {sum_possible} (P: {num_possible}, NP: {num_impossible})");
}
