// baekjoon 2156 포도주 시식
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
    let bigger = |(a, b, c): (usize, usize, usize)| -> usize {
        a.max(b).max(c)
    };
    let mut output = Vec::new();
    let mut main_memo: (usize, usize, usize) = (0, 0, 0);
    let mut sub_memo: (usize, usize, usize) = (0, 0, 0);

    for &v in inputs.iter() {
        if v == 0 {
            let max_val = bigger(sub_memo);
            main_memo = (max_val, max_val, max_val);
        } else {
            main_memo = (sub_memo.1 + v, sub_memo.2 + v, bigger(sub_memo));
        }
        sub_memo = main_memo;
    }
    write!(output, "{}", bigger(main_memo)).unwrap();
    writer.write_all(&output).unwrap();
}