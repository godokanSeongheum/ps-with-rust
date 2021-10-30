use std::{collections::HashMap, fs::File, io::{self, Read, Write, BufWriter}, os::unix::io::FromRawFd, str::from_utf8};
#[derive(Debug)]
struct Counter {
    zero: usize,
    one: usize,
}

fn fibo(n: usize, memo: &mut HashMap<usize, Counter>) -> Counter {
    if n == 0 {
        return Counter {zero: 1, one: 0};
    }
    if n == 1 {
        return Counter {zero: 0, one: 1};
    }
    let counter1 = match memo.get(&(n - 1)) {
        Some(val) => {
            Counter {zero: val.zero, one: val.one}
        },
        None => {
            let counter = fibo(n - 1, memo);
            memo.insert(n - 1, Counter { zero: counter.zero, one: counter.one});
            counter
        },
    };
    let counter2 = match memo.get(&(n - 2)) {
        Some(val) => {
            Counter {zero: val.zero, one: val.one}
        },
        None => {
            let counter = fibo(n - 2, memo);
            memo.insert(n - 2, Counter { zero: counter.zero, one: counter.one});
            counter
        },
    };
    Counter {zero: counter1.zero + counter2.zero, one: counter1.one + counter2.one}
}
fn main() {
    let mut buf_writer = BufWriter::new(unsafe { File::from_raw_fd(1) });
    let nums: Vec<usize> = {
        let mut input = Vec::new();
        io::stdin().read_to_end(&mut input).unwrap();
        from_utf8(&input).unwrap().split_whitespace().skip(1)
            .map(|x| x.parse().unwrap())
            .collect()
    };
    let mut output = Vec::new();
    let mut counter = Counter { zero: 0, one: 0, };
    let mut memo: HashMap<usize, Counter> = HashMap::new();
    for &num in nums.iter() {
        match memo.get(&num) {
            Some(val) => {
                counter.zero += val.zero;
                counter.one += val.one; 
            },
            None => {
                counter = fibo(num, &mut memo)
            },
        }
        memo.insert(num, Counter { zero: counter.zero, one: counter.one});
        write!(output, "{} {}\n", counter.zero, counter.one).unwrap();
        counter.zero = 0;
        counter.one = 0;
    }
    buf_writer.write_all(&output).unwrap();
}