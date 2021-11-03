// beakjoon 1149 RGB 거리
use std::{
    io::{self, Read, Write, BufWriter},
    str::from_utf8,
    os::unix::io::FromRawFd,
    fs::File
};

#[derive(Clone)]
struct MinRGB {
    red_min: usize,
    green_min: usize,
    blue_min: usize,
}

fn solve(
    line: &[usize],
    result: &mut MinRGB) -> () {
    if let [r, g, b] = line[..3] {
        let tmp = result.clone();
        result.red_min = r + tmp.blue_min.min(tmp.green_min);
        result.green_min = g + tmp.red_min.min(tmp.blue_min);
        result.blue_min = b + tmp.red_min.min(tmp.green_min);
    };
}
fn main() {
    let mut writer = BufWriter::new(unsafe {File::from_raw_fd(1)});
    let inputs: Vec<usize> = {
        let mut input = Vec::new();
        io::stdin().read_to_end(&mut input).unwrap();
        from_utf8(&input).unwrap().trim().split_whitespace()
            .skip(1)
            .map(|x| x.parse().unwrap()).collect()
    };
    let mut output = Vec::new();
    let mut result: MinRGB = MinRGB { red_min: 0, blue_min: 0, green_min: 0};
    for line in inputs.chunks(3) {
        solve(line, &mut result);
    }
    write!(output, "{}", result.red_min.min(
        result.blue_min).min(result.green_min)).unwrap();
    writer.write_all(&output).unwrap();
}