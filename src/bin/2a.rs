use advent_of_code_2024::input::get_lines;

fn main() {
    let mut safe: u32 = 0;
    for line in get_lines(2) {
        let levels: Vec<u32> = line
            .expect("Not line")
            .split_whitespace()
            .map(|word| word.parse().expect("Not level encountered"))
            .collect();
        let safe_a = &levels
            .windows(3)
            .all(|w| (w[0] <= w[1] && w[1] <= w[2]) || (w[0] >= w[1] && w[1] >= w[2]));
        let safe_b = &levels.windows(2).all(|w| {
            let diff = w[0].abs_diff(w[1]);
            diff >= 1 && diff <= 3
        });
        safe += if *safe_a && *safe_b { 1 } else { 0 };
    }
    println!("Amount safe: {}", safe);
}
