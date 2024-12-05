use advent_of_code_2024::input::get_lines;

use regex::Regex;

fn main() {
    let mul_regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").expect("wrong regex!");
    let mut total: u32 = 0;
    for line in get_lines(3) {
        let actual_line = line.expect("no line..!");
        let matches = mul_regex.captures_iter(&actual_line);
        for capture_match in matches {
            let a: u32 = capture_match
                .get(1)
                .expect("no capture a!")
                .as_str()
                .parse()
                .expect("capture a no u32!");
            let b: u32 = capture_match
                .get(2)
                .expect("no capture b!")
                .as_str()
                .parse()
                .expect("capture b no u32!");
            total += a * b;
        }
    }
    println!("Total: {}", total);
}
