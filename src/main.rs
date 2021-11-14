// baekjoon 2609 최대공약수와 최소공배수
use std::io;
fn main() {
    let stdin = io::stdin();
    let mut ab = String::new();
    stdin.read_line(&mut ab).unwrap();
    let mut iter = ab.trim().split_whitespace()
        .map(|x| x.parse::<usize>().unwrap());
    let first = iter.next().unwrap();
    let second = iter.next().unwrap();
    let mut a = first;
    let mut b = second;
    let mut gcd: usize = usize::MAX;
    loop {
        if a < b {
            if b % a == 0 {
                gcd = a;
                break;
            } else {
                b = b % a;
            }
        } else {
            if a % b == 0 {
                gcd = b;
                break;
            } else {
                a = a % b;
            }
        }
    }
    println!("{}\n{}", gcd, first * second / gcd);
}