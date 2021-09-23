use std::{io::{ self, Read }, str::from_utf8};
fn main() {
    let input: Vec<Vec<u32>> = {
        let mut stdin: Vec<u8> = Vec::new();
        io::stdin().read_to_end(&mut stdin).unwrap();
        from_utf8(&stdin).unwrap()
            .trim().split('\n')
            .map(|x| -> Vec<u32> {
                x.trim().split_whitespace()
                    .map(|y| -> u32 {y.parse().unwrap()})
                    .collect()
            }).skip(1)
            .collect()
    };
    let mut result: Vec<String> = Vec::new();
    for i in 0..input.len() {
        let mut count = 1;
        for j in 0..input.len() {
            if input[i][0] < input[j][0] && input[i][1] < input[j][1] {
                count += 1;
            }
        }
        result.push(format!("{}", count));
    }
    println!("{}", result.join(" "));
}
