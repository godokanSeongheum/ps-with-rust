// baekjoon 1037 약수
use std::io;
fn main() {
    let stdin = io::stdin();
    let mut n = String::new();
    let mut nums = String::new();
    stdin.read_line(&mut n).unwrap();
    stdin.read_line(&mut nums).unwrap();
    let nums: Vec<usize> = nums.trim().split_whitespace()
        .map(|x| x.parse().unwrap()).collect();
    let mut min_val = usize::MAX;
    let mut max_val = usize::MIN;
    for num in nums {
        if num < min_val {
            min_val = num;
        }
        if num > max_val {
            max_val = num;
        }
    }
    println!("{}", min_val * max_val);
}