use advent_of_code_2024::input::get_lines;

use itertools::iproduct;

fn check_xmas(board: &Vec<String>, loc: [i32; 2], dir: [i32; 2]) -> bool {
    if dir[0] == 0 && dir[1] == 0 {
        return false;
    }
    let width = board[0].len() as i32;
    let height = board.len() as i32;
    let xmas = [b'X', b'M', b'A', b'S'];
    for i in 0..4 {
        let x = loc[0] + i * dir[0];
        let y = loc[1] + i * dir[1];
        if x < 0 || x >= width {
            return false;
        }
        if y < 0 || y >= height {
            return false;
        }
        if board[y as usize].as_bytes()[x as usize] != xmas[i as usize] {
            return false;
        }
    }
    return true;
}

fn main() {
    let mut board: Vec<String> = Vec::new();
    for line in get_lines(4) {
        board.push(line.expect("Not a line..!"));
    }

    let width = board[0].len() as i32;
    let height = board.len() as i32;

    let mut total = 0;
    for p in iproduct!(0..width, 0..height) {
        for d in iproduct!(-1..2, -1..2) {
            total += if check_xmas(&board, p.into(), d.into()) {
                1
            } else {
                0
            };
        }
    }
    println!("Total: {}", total);
}
