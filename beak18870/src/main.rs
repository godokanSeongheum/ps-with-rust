use std::collections::HashMap;
use std::io::{self, Read, BufWriter, Write};
use std::str::from_utf8;
use std::fs::File;
use std::os::unix::io::FromRawFd;
fn main() {
    let nums: Vec<isize> = {
        let mut input = Vec::new();
        io::stdin().read_to_end(&mut input).unwrap();
        from_utf8(&input).unwrap().trim().split_whitespace()
            .skip(1).map(|x| x.parse().unwrap())
            .collect()
    };
    let mut hm: HashMap<isize, usize> = HashMap::new();
    let mut sorted_nums = nums.clone();
    sorted_nums.sort();
    sorted_nums.dedup();
    for (ziped_num, &original_num) in sorted_nums.iter().enumerate() {
        hm.insert(original_num, ziped_num);
    }
    let mut writer = BufWriter::new(unsafe { File::from_raw_fd(1) });
    let mut output = Vec::new();

    for num in nums {
        write!(output, "{} ", hm.get(&num).unwrap()).unwrap();
    }
    writer.write_all(&output).unwrap();
}
