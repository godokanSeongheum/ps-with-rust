// baekjoon 2981 검문
use std::{fs::File, io::{self, BufWriter, Read, Write}, os::unix::prelude::FromRawFd, str::from_utf8};

fn get_gcd(first: i64, second: i64) -> i64 {
    let mut a = first;
    let mut b = second;
    let mut gcd: i64 = i64::MAX;
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
    let input: Vec<i64> = from_utf8(&input).unwrap().trim().split_whitespace()
        .skip(1).map(|x| x.parse::<i64>().unwrap()).collect();
    let val = input[0];
    let mut tmp = input[1];
    let mut gcd = (tmp - val).abs();
    for &val in input.iter().skip(2) {
        gcd = get_gcd((tmp - val).abs(), gcd);
        tmp = val;
    }
    let mut output = Vec::new();
    for num in 2..=gcd / 2 {
        if gcd % num == 0 {
            write!(output, "{} ", num).unwrap();
        }
    }
    write!(output, "{}", gcd).unwrap();
    writer.write_all(&output).unwrap();
}