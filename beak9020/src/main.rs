use std::{io, str::from_utf8};
use std::io::Read;
fn main() {
    const _MAX_N: usize = 10000;
    let mut input: Vec<u8> = Vec::new();
    io::stdin().read_to_end(&mut input).unwrap();
    let input: Vec<usize> = from_utf8(&input).unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap()).collect();
    let mut all: Vec<bool> = vec![true;_MAX_N + 1];

    for i in 2..=_MAX_N {
        if all[i] {
            let mut j = 2;
            while i * j <= _MAX_N {
                all[i * j] = false;
                j += 1;
            }
        }
    }
    let primes: Vec<usize> = all.iter().enumerate()
        .filter(|&(_, &v)| v)
        .map(|(i, _)| i).collect();
    for &t in &input[1..] {
        let mut less: usize = 0;
        let mut greater: usize = 0;
        for &p in primes.iter() {
            if t < p * 2 {
                break;
            }
            if all[t - p] {
                less = p;
                greater = t - p;
            }
        }
        println!("{} {}", less, greater);
    }
}
