// beakjoon 1463 1로 만들기
use std::{
    io::{self, Write, BufWriter},
    os::unix::io::FromRawFd,
    fs::File
};
fn main() {
    let mut writer = BufWriter::new(unsafe {File::from_raw_fd(1)});
    let x: usize = {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().parse().unwrap()
    };
    let mut output = Vec::new();
    let mut table: [usize;1_000_001] = [0;1_000_001];
    table[2] = 1;
    table[3] = 1;
    for i in 4..1_000_001 {
        if i % 2 == 0 && i % 3 == 0 {
            table[i] = table[i / 2].min(table[i / 3]).min(table[i - 1]) + 1;
            continue;
        }
        if i % 3 == 0 {
            table[i] = table[i / 3].min(table[i - 1]) + 1;
            continue;
        }
        if i % 2 == 0 {
            table[i] = table[i / 2].min(table[i - 1]) + 1;
            continue;
        }
        table[i] = table[i - 1] + 1;
    }

    write!(output, "{:?}", table[x]).unwrap();
    writer.write_all(&output).unwrap();
}