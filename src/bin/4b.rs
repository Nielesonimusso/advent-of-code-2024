use advent_of_code_2024::input::get_lines;

use itertools::iproduct;

fn board_char(board: &Vec<String>, loc: [i32; 2]) -> char {
    let width = board[0].len() as i32;
    let height = board.len() as i32;

    if loc[0] < 0 || loc[0] >= width {
        return 'Q';
    }
    if loc[1] < 0 || loc[1] >= height {
        return 'Q';
    }
    board[loc[1] as usize].as_bytes()[loc[0] as usize] as char
}

fn check_x_mas(board: &Vec<String>, loc: [i32; 2], dir: [i32; 2]) -> bool {
    if (dir[0] == 0 && dir[1] == 0) || (dir[0] != 0 && dir[1] != 0) {
        return false;
    }
    if board_char(board, loc) != 'A' {
        return false;
    }
    let horizontal = dir[0] != 0;

    match horizontal {
        true => {
            board_char(board, [loc[0] - dir[0], loc[1] - 1]) == 'M'
                && board_char(board, [loc[0] - dir[0], loc[1] + 1]) == 'M'
                && board_char(board, [loc[0] + dir[0], loc[1] - 1]) == 'S'
                && board_char(board, [loc[0] + dir[0], loc[1] + 1]) == 'S'
        }
        false => {
            board_char(board, [loc[0] - 1, loc[1] - dir[1]]) == 'M'
                && board_char(board, [loc[0] + 1, loc[1] - dir[1]]) == 'M'
                && board_char(board, [loc[0] - 1, loc[1] + dir[1]]) == 'S'
                && board_char(board, [loc[0] + 1, loc[1] + dir[1]]) == 'S'
        }
    }
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
            if check_x_mas(&board, p.into(), d.into()) {
                total += 1;
            };
        }
    }
    println!("Total: {}", total);
}
