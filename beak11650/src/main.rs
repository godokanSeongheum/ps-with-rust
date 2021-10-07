use std::{io::{self, BufWriter, Read}, str::from_utf8};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::os::unix::io::FromRawFd;
fn main() {
    let coords: Vec<i32> = {
        let mut input: Vec<u8> = Vec::new();
        io::stdin().read_to_end(&mut input).unwrap();
        from_utf8(&input).unwrap().trim().split_whitespace().skip(1)
            .map(|x| x.parse().unwrap()).collect()
    };
    let mut hm: HashMap<i32, Vec<i32>> = HashMap::new();
    for idx in (0..coords.len()).step_by(2) {
        hm.entry(coords[idx]).or_insert(Vec::new()).push(coords[idx + 1]);
    }
    let mut writer = BufWriter::new(unsafe { File::from_raw_fd(1) });

    let mut output = Vec::new();
    for num in -100_000..=100_000 {
        if hm.contains_key(&num) {
            let v = hm.get_mut(&num).unwrap();
            v.sort();
            for k in v {
                write!(output, "{} {}\n", num, k).unwrap();
            }
        }
    }
    writer.write_all(&output).unwrap();
}
