// beakjoon 10844 쉬운 계단 수
use std::{
    io::{self, Write, BufWriter},
    os::unix::io::FromRawFd,
    fs::File
};
fn main() {
    let mut writer = BufWriter::new(unsafe {File::from_raw_fd(1)});
    let n: usize = {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().parse::<usize>().unwrap() - 1
    };
    let mut output = Vec::new();
    let mut table: [[u64;10];100] = [[0;10];100]; 
    table[0] = [0, 1, 1, 1, 1, 1, 1, 1, 1, 1];
    for i in 1..=n {
        table[i][0] = table[i - 1][1];
        table[i][9] = table[i - 1][8];
        for digit in 1..9 {
            table[i][digit] = (table[i - 1][digit - 1] + table[i - 1][digit + 1]) %
                                                1_000_000_000;
        }
    }
    write!(output, "{}", table[n].iter().sum::<u64>() % 1_000_000_000).unwrap();
    writer.write_all(&output).unwrap();
}