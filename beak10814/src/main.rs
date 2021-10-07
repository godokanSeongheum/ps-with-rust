use std::{io::{self, Read}, str::from_utf8};
use std::io::BufWriter;
use std::fs::File;
use std::os::unix::io::FromRawFd;
use std::io::Write;
fn main() {
    let elements: Vec<String> = {
        let mut input: Vec<u8> = Vec::new();
        io::stdin().read_to_end(&mut input).unwrap();
        from_utf8(&input).unwrap().trim().split_whitespace()
            .skip(1).map(|x| x.to_string()).collect()
    };
    let mut infos: Vec<(usize, u32, String)> = Vec::new();
    for idx in (0..elements.len()).step_by(2) {
        infos.push((idx, elements[idx].parse().unwrap(), elements[idx + 1].clone()));
    }
    infos.sort_by_key(|tup| tup.0);
    infos.sort_by_key(|tup| tup.1);
    let mut writer = BufWriter::new(unsafe { File::from_raw_fd(1) });
    let mut output = Vec::new();
    for info in infos {
        write!(output, "{} {}\n", info.1, info.2).unwrap();
    }
    writer.write_all(&output).unwrap();
}
