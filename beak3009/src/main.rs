use std::io;
use std::io::Read;
use std::str::from_utf8;

fn main() {
    let mut input: Vec<u8> = Vec::new();
    io::stdin().read_to_end(&mut input).unwrap();
    let input: Vec<Vec<i32>> = from_utf8(&input).unwrap()
        .split('\n').map(|x| x.split_whitespace()
            .map(|y| y.parse::<i32>().unwrap()).collect())
            .collect();
    let mut x = 0;
    let mut y = 0;

    if input[0][0] == input[1][0] && input[0][1] == input[2][1] {
        x = input[2][0];
        y = input[1][1];
    }
    if input[0][0] == input[2][0] && input[0][1] == input[1][1] {
        x = input[1][0];
        y = input[2][1];
    }


    if input[1][0] == input[0][0] && input[1][1] == input[2][1] {
        x = input[2][0];
        y = input[0][1];
    }
    if input[1][0] == input[2][0] && input[1][1] == input[0][1] {
        x = input[0][0];
        y = input[2][1];
    }

    if input[2][0] == input[1][0] && input[2][1] == input[0][1] {
        x = input[0][0];
        y = input[1][1];
    }
    if input[2][0] == input[0][0] && input[2][1] == input[1][1] {
        x = input[1][0];
        y = input[0][1];
    }
    println!("{} {}", x, y);
}
