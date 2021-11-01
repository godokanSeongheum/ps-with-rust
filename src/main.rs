// beakjoon 9461 파도반 수열
use std::{
    io::{self, Read, Write, BufWriter},
    str::from_utf8,
    os::unix::io::FromRawFd,
    fs::File,
    collections::HashMap
};

fn get_answer(n: isize, memo: &mut HashMap<isize, isize>) -> isize {
    match memo.get(&n) {
        Some(&answer) => answer,
        None => {
            let answer = solve(n, memo);
            memo.insert(n, answer);
            answer
        },
    }
}

fn solve(n: isize, memo: &mut HashMap<isize, isize>) -> isize {
    if n > 0 && n < 4 {
        1
    } else if n == 4 {
        2
    } else if n <= 0 {
        0
    } else {
        get_answer(n - 1, memo) + get_answer(n - 5, memo)
    }
}
fn main() {
    let mut writer = BufWriter::new(unsafe {File::from_raw_fd(1)});
    let inputs: Vec<isize> = {
        let mut input = Vec::new();
        io::stdin().read_to_end(&mut input).unwrap();
        from_utf8(&input).unwrap().trim().split_whitespace()
            .skip(1)
            .map(|x| x.parse().unwrap()).collect()
    };
    let mut output = Vec::new();
    let mut memo: HashMap<isize, isize> = HashMap::new();
    for n in inputs.iter() {
        let answer = match memo.get(n) {
            Some(&n) => n,
            None => solve(*n, &mut memo),
        };
        write!(output, "{}\n", answer).unwrap();
    }
    writer.write_all(&output).unwrap();
}