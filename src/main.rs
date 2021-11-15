// baekjoon 1934 최소공배수
use std::{io::{self, Read}, str::from_utf8};

fn get_gcd(first: usize, second: usize) -> usize {
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
    gcd
}
fn main() {
    let mut stdin = io::stdin();
    let mut input = Vec::new();
    stdin.read_to_end(&mut input).unwrap();
    let mut iter = from_utf8(&input).unwrap().trim().split_whitespace()
        .skip(1);
    loop {
        let first: usize = match iter.next() {
            Some(str) => str.parse().unwrap(),
            None => break,
        };
        let second: usize = match iter.next() {
            Some(str) => str.parse().unwrap(),
            None => break,
        };
        println!("{}", first * second / get_gcd(first, second));
    }
}