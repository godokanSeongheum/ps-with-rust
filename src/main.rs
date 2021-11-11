// baekjoon 12865 평범한 배낭
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
    let mut items: [(usize, usize);101] = [(0, 0);101];
    let k = inputs[1];
    for (i, chunk) in inputs.chunks(2).skip(1).enumerate() {
        items[i].0 = chunk[0]; items[i].1 = chunk[1];
    }
    items.sort_by_key(|(w, _)| *w);

    let mut dp: Vec<[usize;100_001]> = vec![[0;100_001];items.len() + 1];
    for (i, item) in items.iter().enumerate() {
        for w in 1..=k {
            dp[i + 1][w] = dp[i][w];
            if w >= item.0 {
                dp[i + 1][w] = dp[i + 1][w].max(dp[i][w - item.0] + item.1);
            }
        }
    }


    let mut output = Vec::new();
    let result = dp[items.len()][k];
    write!(output, "{}", result).unwrap();
    writer.write_all(&output).unwrap();
}