// baekjoon 11053 가장 긴 증가하는 부분 수열
use std::{fs::File, io::{self, Read, Write, BufWriter}, os::unix::io::FromRawFd, str::from_utf8};
fn main() {
    let mut writer = BufWriter::new(unsafe {File::from_raw_fd(1)});
    let inputs: Vec<usize>  = {
        let mut input = Vec::new();
        io::stdin().read_to_end(&mut input).unwrap();
        from_utf8(&input).unwrap().trim().split_whitespace()
            .skip(1).map(|x| x.parse().unwrap())
            .collect()
    };
    let mut output = Vec::new();
    let mut memo: [usize;1000] = [0;1000];
    for i in 0..inputs.len() {
        let max_val_idx = inputs[0..i].iter().enumerate()
            .filter(|(_, &v)| v < inputs[i])
            .map(|(idx, _)| idx)
            .reduce(|a, b| if memo[a] > memo[b] {a} else {b});
        match max_val_idx {
            Some(idx) => memo[i] = memo[idx] + 1,
            None => memo[i] = 1,
        }
    }
    write!(output, "{}", memo.iter().max().unwrap()).unwrap();
    writer.write_all(&output).unwrap();
}