// beakjoon 2579 계단 오르기
use std::{
    io::{self, Read, Write, BufWriter},
    str::from_utf8,
    os::unix::io::FromRawFd,
    fs::File
};
fn main() {
    let mut writer = BufWriter::new(unsafe {File::from_raw_fd(1)});
    let inputs: Vec<usize> = {
        let mut input = Vec::new();
        io::stdin().read_to_end(&mut input).unwrap();
        from_utf8(&input).unwrap().trim().split_whitespace()
            .skip(1)
            .map(|x| x.parse().unwrap())
            .collect()
    };
    let mut memo: [(usize, usize);300] = [(0, 0);300];
    let mut output = Vec::new();
    for (idx, &num) in inputs.iter().enumerate() {
        if idx == 0 {
            memo[0] = (num, 0);
            continue;
        }
        if idx == 1 {
            memo[1] = (memo[0].0 + num, num);
            continue;
        }
        memo[idx] = (
            memo[idx - 1].1 + num,
            memo[idx - 2].0.max(memo[idx - 2].1) + num
        );
    }
    let result = memo[inputs.len() - 1].0.max(memo[inputs.len() - 1].1);
    write!(output, "{}", result).unwrap();
    writer.write_all(&output).unwrap();
}