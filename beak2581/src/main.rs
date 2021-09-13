use std::{io::{self, Read}, str::from_utf8};
fn main() {
    const _MAX_VALUE: usize = 10001;
    let mut input: Vec<u8> = Vec::new();
    io::stdin().read_to_end(&mut input).unwrap();
    let input: Vec<usize> = from_utf8(&input).unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap()).collect();
    let mut all_value: [bool;_MAX_VALUE] = [true;_MAX_VALUE];
    all_value[1] = false;
    for i in 2.._MAX_VALUE {
        if all_value[i] {
            let mut j = 2;
            while i * j < _MAX_VALUE {
                all_value[i * j] = false;
                j += 1;
            }
        }
    }
    let mut answer: usize = 0;
    let mut min: usize = 0;
    for i in (input[0]..=input[1]).rev() {
        if all_value[i] {
            answer += i;
            min = i;
        }
    }
    if answer == 0 {
        println!("-1");
    } else {
        println!("{}", answer);
        println!("{}", min);
    }
}
