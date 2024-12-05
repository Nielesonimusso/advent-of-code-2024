use advent_of_code_2024::input::get_lines;

fn main() {
    let mut safe_count: u32 = 0;
    let predicate_a = |w: &[u32]| (w[0] <= w[1] && w[1] <= w[2]) || (w[0] >= w[1] && w[1] >= w[2]);
    let predicate_b = |w: &[u32]| {
        let diff = w[0].abs_diff(w[1]);
        diff >= 1 && diff <= 3
    };
    for line in get_lines(2) {
        let levels: Vec<u32> = line
            .expect("Not line")
            .split_whitespace()
            .map(|word| word.parse().expect("Not level encountered"))
            .collect();
        match *&levels.windows(3).all(predicate_a) && *&levels.windows(2).all(predicate_b) {
            true => safe_count += 1,
            false => {
                let level_slices: Vec<Vec<u32>> = (0..levels.len())
                    .map(|i| {
                        let mut slice = levels.clone();
                        slice.remove(i);
                        slice
                    })
                    .collect();
                safe_count += if level_slices.iter().any(|level_slice| {
                    *&level_slice.windows(3).all(predicate_a)
                        && *&level_slice.windows(2).all(predicate_b)
                }) {
                    1
                } else {
                    0
                };
            }
        };
    }
    println!("Amount safe: {}", safe_count);
}
