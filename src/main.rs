// baekjoon 11054 가장 긴 바이토닉 부분 수열
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
    let mut memo: [(usize, usize);1000] = [(0, 0);1000];
    // right
    for i in 0..inputs.len() {
        let max_val_idx = inputs[0..i].iter().enumerate()
            .filter(|(_, &v)| v < inputs[i])
            .map(|(idx, _)| idx)
            .reduce(|a, b| if memo[a].0 > memo[b].0 {a} else {b});
        match max_val_idx {
            Some(idx) => memo[i].0 = memo[idx].0 + 1,
            None => memo[i].0 = 1,
        }
    }

    // reverse
    for i in (0..inputs.len()).rev() {
        let slice: Vec<(usize, usize)> = inputs.iter().enumerate()
            .filter(|(idx, _)| i < *idx).map(|(i, v)| (i, *v)).collect();
        let max_val_idx = slice.iter().filter(|(_, v)| *v < inputs[i])
            .map(|(idx, _)| idx)
            .reduce(|a, b| if memo[*a].1 > memo[*b].1 {a} else {b});
        match max_val_idx {
            Some(idx) => memo[i].1 = memo[*idx].1 + 1,
            None => memo[i].1 = 1,
        }
    }
    let result = memo[..inputs.len()].iter()
        .map(|(a, b)| a + b - 1).max().unwrap();
    write!(output, "{}", result).unwrap();
    writer.write_all(&output).unwrap();
}