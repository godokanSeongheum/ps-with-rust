// baekjoon 5086 배수와 약수
use std::io;
fn main() {
    let stdin = io::stdin();
    let mut line = String::new();
    loop {
        stdin.read_line(&mut line).unwrap();
        let mut iter = line.trim().split_whitespace()
            .map(|x| x.parse::<usize>().unwrap());
        if let Some(a) = iter.next() {
            if let Some(b) = iter.next() {
                if a == 0 && b == 0 {
                    break;
                }
                if b % a == 0 {
                    println!("factor");
                } else if a % b == 0 {
                    println!("multiple");
                } else {
                    println!("neither");
                }
            }
        } else {
            break;
        }
        line.clear();
    }
}