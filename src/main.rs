use std::{io::{self, BufWriter, Read, Write}, str::from_utf8};
use std::os::unix::io::FromRawFd;
use std::fs::File;
// beakjoon 15652
fn permute(
    start: usize, 
    n: usize,
    m: usize,
    output: &mut Vec<u8>,
    tmparr:&Vec<usize>,
) -> () {
    if m == 0 {
        let chunk = tmparr.iter().map(|x| x.to_string())
            .collect::<Vec<String>>().join(" ");
        write!(output, "{}\n", chunk).unwrap();
        return;
    }
    for num in start..=n {
        let mut next_tmparr = tmparr.clone();
        next_tmparr.push(num);
        permute(num, n, m - 1, output, &next_tmparr);
    }
    return;
}
fn main() {
    let nm: Vec<usize> = {
        let mut input = Vec::new();
        io::stdin().read_to_end(&mut input).unwrap();
        from_utf8(&input).unwrap().split_whitespace()
            .map(|x| x.parse().unwrap()).collect()
    };
    let n = nm[0];
    let m = nm[1];
    let mut output = Vec::new();
    let mut writer = BufWriter::new(unsafe {File::from_raw_fd(1)});
    let v = Vec::new();
    permute(1, n, m, &mut output, &v);
    drop(nm);
    writer.write_all(&output).unwrap();
}