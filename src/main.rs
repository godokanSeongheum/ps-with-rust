// baekjoon 3036 링
use std::{fs::File, io::{self, BufWriter, Read, Write}, os::unix::prelude::FromRawFd, str::from_utf8};

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
    let mut writer = BufWriter::new(unsafe {File::from_raw_fd(1)});
    let mut stdin = io::stdin();
    let mut input = Vec::new();
    stdin.read_to_end(&mut input).unwrap();
    let input: Vec<usize> = from_utf8(&input).unwrap().trim().split_whitespace()
        .skip(1).map(|x| x.parse::<usize>().unwrap()).collect();
    let start = input[0];
    let mut output = Vec::new();
    for &num in input.iter().skip(1) {
        let gcd = get_gcd(start, num);
        write!(output, "{}/{}\n", start / gcd, num / gcd).unwrap();
    }
    writer.write_all(&output).unwrap();
}