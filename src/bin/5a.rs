use advent_of_code_2024::input::get_lines;

use once_cell;
use regex;
use std::collections::HashMap;

macro_rules! regex {
    ($re:literal $(,)?) => {{
        static RE: once_cell::sync::OnceCell<regex::Regex> = once_cell::sync::OnceCell::new();
        RE.get_or_init(|| regex::Regex::new($re).unwrap())
    }};
}

fn parse_line(line: &String) -> Option<(u8, u8)> {
    let rule_regex: &regex::Regex = regex!(r"(?<lhs>\d+)\|(?<rhs>\d+)");
    if line.len() > 0 {
        let rule_match = rule_regex.captures(&line).expect("No rule..!");
        let lhs: u8 = rule_match
            .name("lhs")
            .expect("No lhs..!")
            .as_str()
            .parse()
            .expect("No lhs number..!");
        let rhs: u8 = rule_match
            .name("rhs")
            .expect("No rhs..!")
            .as_str()
            .parse()
            .expect("No rhs number..!");
        Some((lhs, rhs))
    } else {
        None
    }
}

fn insert_ordered_pair_into_rules(
    pair: (u8, u8),
    before_rules: &mut HashMap<u8, Vec<u8>>,
    after_rules: &mut HashMap<u8, Vec<u8>>,
) {
    before_rules
        .entry(pair.1)
        .and_modify(|v| v.push(pair.0))
        .or_insert(vec![pair.0]);
    after_rules
        .entry(pair.0)
        .and_modify(|v| v.push(pair.1))
        .or_insert(vec![pair.1]);
}

fn comparator(
    a: &u8,
    b: &u8,
    before_rules: &HashMap<u8, Vec<u8>>,
    after_rules: &HashMap<u8, Vec<u8>>,
) -> bool {
    match before_rules.get(b) {
        Some(v) => v.contains(a),
        None => match after_rules.get(a) {
            Some(v) => !v.contains(b),
            None => true,
        },
    }
}

fn main() {
    let mut before_rules: HashMap<u8, Vec<u8>> = HashMap::new();
    let mut after_rules: HashMap<u8, Vec<u8>> = HashMap::new();

    let mut lines = get_lines(5);

    lines
        .by_ref()
        .map_while(|line_e| parse_line(&line_e.expect("No line!")))
        .for_each(|pair| insert_ordered_pair_into_rules(pair, &mut before_rules, &mut after_rules));

    let mut middles_sum: u32 = 0;

    for line_e in lines {
        let line = line_e.expect("No line..!");
        let update: Vec<u8> = line
            .split(',')
            .map(|e| e.parse().expect("Non-number update..!"))
            .collect();
        if update.is_sorted_by(|a, b| comparator(a, b, &before_rules, &after_rules)) {
            middles_sum += update[(update.len() - 1) / 2] as u32;
        }
    }
    println!("Sum of middles: {}", middles_sum);
}
