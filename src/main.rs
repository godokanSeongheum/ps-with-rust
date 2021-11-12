// baekjoon 1931 회의실 배정
use std::{fs::File, io::{self, Read, Write, BufWriter}, os::unix::io::FromRawFd, str::from_utf8};
fn main() {
    let mut writer = BufWriter::new(unsafe {File::from_raw_fd(1)});
    let mut inputs: Vec<(usize, usize)>  = {
        let mut input = Vec::new();
        io::stdin().read_to_end(&mut input).unwrap();
        from_utf8(&input).unwrap().trim().split("\n")
            .skip(1).map(|x| {
                let mut line = x.split_whitespace()
                    .map(|y| y.parse().unwrap());
                (line.next().unwrap(), line.next().unwrap())
            })
            .collect()
    };
    let mut output = Vec::new();
    inputs.sort_by_key(|x| x.1);
    inputs.sort_by_key(|x| x.0);
    let mut count = 0;
    let mut prev = (0, 0);
    for i in 0..inputs.len() {
        if prev.1 <= inputs[i].0 {   // safe
            count += 1;
            prev = inputs[i];
        } else {
            if prev.1 >= inputs[i].1 {
                prev = inputs[i];
            }
        }
    }
    let result = count;
    write!(output, "{}", result).unwrap();
    writer.write_all(&output).unwrap();
}

// inputs[i]를 평가
