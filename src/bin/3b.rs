use advent_of_code_2024::input::get_lines;

use regex::Regex;

fn main() {
    let cond_mul_regex = Regex::new(r"(?<op>mul|do(?:n't)?)\((?:(?<a>\d{1,3}),(?<b>\d{1,3}))?\)")
        .expect("wrong regex!");
    let mut total: u32 = 0;
    let mut enabled: bool = true;
    for line in get_lines(3) {
        let actual_line = line.expect("no line..!");
        let matches = cond_mul_regex.captures_iter(&actual_line);
        for capture_match in matches {
            match capture_match.name("op").expect("no op!").as_str() {
                "mul" => {
                    if enabled {
                        let a: u32 = capture_match
                            .name("a")
                            .expect("no a!")
                            .as_str()
                            .parse()
                            .expect("a no u32!");
                        let b: u32 = capture_match
                            .name("b")
                            .expect("no b!")
                            .as_str()
                            .parse()
                            .expect("b no u32!");
                        total += a * b;
                    }
                }
                "do" => enabled = true,
                "don't" => enabled = false,
                _ => panic!("weird op!"),
            };
        }
    }
    println!("Total: {}", total);
}
