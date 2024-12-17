use advent_of_code_2024::input::get_lines;

use regex::Regex;

fn check_operators(target: u64, accumulator: u64, remaining: &[u64]) -> bool {
    if !remaining.is_empty() && accumulator < target {
        let next_remaining = &remaining[1..];
        return check_operators(target, accumulator + remaining[0], next_remaining)
            || check_operators(target, accumulator * remaining[0], next_remaining);
    } else {
        let result = accumulator == target;
        return result;
    }
}

fn main() {
    let line_regex = Regex::new(r"(?<target>\d+):(?<inputs>(?:\s+\d+)+)").expect("wrong regex!");

    let mut sum_possible: u64 = 0;
    let mut lines = get_lines(7);
    while let Some(Ok(line)) = lines.next() {
        if let Some(captures) = line_regex.captures(&line) {
            let target: u64 = captures
                .name("target")
                .and_then(|mat| mat.as_str().parse().ok())
                .expect("no target..!");
            let inputs: Vec<u64> = captures
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
            if check_operators(target, 0, &inputs[..]) {
                sum_possible += target;
            }
        }
    }

    println!("Sum of possibles: {sum_possible}");
}
