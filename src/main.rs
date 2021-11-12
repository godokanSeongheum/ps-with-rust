// baekjoon 11047 동전 0
use std::{fs::File, io::{self, Read, Write, BufWriter}, os::unix::io::FromRawFd, str::from_utf8};
fn main() {
    let mut writer = BufWriter::new(unsafe {File::from_raw_fd(1)});
    let inputs: Vec<usize>  = {
        let mut input = Vec::new();
        io::stdin().read_to_end(&mut input).unwrap();
        from_utf8(&input).unwrap().trim().split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect()
    };
    let mut output = Vec::new();
    let n = inputs[0];
    let mut k = inputs[1];
    let mut count = 0;
    for i in (2..n + 2).rev() {
        let div = k / inputs[i];
        if div != 0 {
            count += div;
            k %= inputs[i]
        }
    }
    
    let result = count;
    write!(output, "{}", result).unwrap();
    writer.write_all(&output).unwrap();
}