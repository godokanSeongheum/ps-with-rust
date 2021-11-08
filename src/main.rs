// baekjoon 2565 전깃줄
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

    let mut arr: [usize;501] = [0;501];
    for slice in inputs.chunks(2) {
        arr[slice[0]] = slice[1];
    }
    let arr: Vec<usize> = arr.iter()
        .filter(|&x| *x != 0).map(|x| *x).collect();
    let mut memo: Vec<usize> = vec![0;arr.len()];
    for i in 0..arr.len() {
        let mut max_val = 0;
        for j in 0..i {
            if arr[i] > arr[j] && memo[j] > max_val {
                max_val = memo[j];
            }
        }
        memo[i] = max_val + 1;
    }

    let result: usize = arr.len() - *(memo.iter().max().unwrap());
    write!(output, "{}", result).unwrap();
    writer.write_all(&output).unwrap();
}