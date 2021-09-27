use std::io::{ self, Read};
use std::str::from_utf8;


const B: char = 'B';
const W: char = 'W';

fn count_of_changes(
    board: &Vec<Vec<char>>, 
    col_start: usize, 
    row_start: usize, 
    is_black: bool) -> u32 {
    let mut count = 0;
    let mut is_black: bool = !is_black;
    for c in col_start..col_start + 8 {
        is_black = !is_black;
        for r in row_start..row_start + 8 {
            if (!is_black && board[c][r] == B) || (is_black && board[c][r] == W) {
                count += 1;
            }
            is_black = !is_black;
        }
    }
    count
}

fn main() {
    let mut input:Vec<u8> = Vec::new();
    io::stdin().read_to_end(&mut input).unwrap();
    let board: Vec<Vec<char>> = from_utf8(&input).unwrap().trim().split('\n')
        .skip(1).map(|s| -> Vec<char> { s.chars().collect() })
        .collect();
    let mut minimum = 64;
    for i in 0..=board.len() - 8 {
        for j in 0..=board[0].len() - 8 {
            let starts_with_black = count_of_changes(&board, i, j, true);
            let starts_with_white = count_of_changes(&board, i, j, false);
            minimum = *[starts_with_black, starts_with_white, minimum].iter().min().unwrap();
        }
    }
    println!("{}", minimum);
}