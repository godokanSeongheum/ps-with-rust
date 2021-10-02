use std::io;
use std::io::Stdin;
use std::io::BufWriter;
use std::io::Write;

use std::fs::File;

use std::os::unix::io::FromRawFd;

fn main() {
    let mut buf_writer = BufWriter::new(unsafe { File::from_raw_fd(1) });
    let reader = io::stdin();
    let read_element = |reader: &Stdin| -> u32 {
        let mut input = String::new();
        reader.read_line(&mut input).unwrap();
        input.trim().parse().unwrap()
    };
    
    let n: u32 = read_element(&reader);
    let mut result: [u32; 10_001] = [0; 10_001];
    for _ in 0..n {
        result[read_element(&reader) as usize] += 1;
    }
    
    for (i, &v) in result.iter().enumerate() {
        let mut output = Vec::new();
        for _ in 0..v {
            write!(output, "{}\n", i).unwrap();
            if output.len() > 500_000 {
                buf_writer.write_all(&output).unwrap();
                output.clear();
            }
        }
        buf_writer.write_all(&output).unwrap();
    }
}
