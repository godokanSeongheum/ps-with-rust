use std::{io::{self, Read}, str::from_utf8};
fn main() {
    const _MAX_2N: usize = 123_456 * 2;
    let mut input: Vec<u8> = Vec::new();
    io::stdin().read_to_end(&mut input).unwrap();
    let input: Vec<usize> = from_utf8(&input).unwrap()
        .split_whitespace()
        .map(|x| x.trim().parse().unwrap())
        .collect();
    let mut all = vec![true;_MAX_2N + 1];
    all[1] = false;
    for k in 2..=_MAX_2N {
        if all[k] {
            let mut q = 2;
            while k * q <= _MAX_2N {
                all[k * q] = false;
                q += 1;
            }
        }
    }

    let mut result: Vec<String> = Vec::new();
    for t in input[..input.len() - 1].iter() {
        result.push(all[*t + 1..=*t * 2].iter().filter(|x| **x).count().to_string());
    }
    println!("{}", result.join("\n"));
}
