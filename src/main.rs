// beakjoon 1932 정수 삼각형
use std::{
    io::{self, Read, Write, BufWriter},
    str::from_utf8,
    os::unix::io::FromRawFd,
    fs::File
};
fn main() {
    let mut writer = BufWriter::new(unsafe {File::from_raw_fd(1)});
    let inputs: Vec<Vec<usize>> = {
        let mut input = Vec::new();
        io::stdin().read_to_end(&mut input).unwrap();
        from_utf8(&input).unwrap().trim().split("\n")
            .skip(1)
            .map(|x| {
                x.trim().split_whitespace()
                    .map(|y| y.parse().unwrap())
                    .collect()
            })
            .collect()
    };
    let mut output = Vec::new();
    let mut arr: [[usize;500];500] = [[10000;500];500];
    for (idx, &val) in inputs[inputs.len() - 1].iter().enumerate() {
        arr[0][idx] = val;
    }
    for (idx,line) in inputs.iter().rev().skip(1).enumerate() {
        for (i, &v) in line.iter().enumerate() {
            arr[idx + 1][i] = arr[idx][i].max(arr[idx][i + 1]) + v;
        }
    }
    write!(output, "{}", arr[inputs.len() - 1][0]).unwrap();
    writer.write_all(&output).unwrap();
}