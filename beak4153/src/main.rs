use std::io::{ self, Read };
use std::str::from_utf8;
fn main() {
    let mut input: Vec<u8> = Vec::new();
    io::stdin().read_to_end(&mut input).unwrap();
    let mut testcases: Vec<Vec<u64>> = from_utf8(&input).unwrap().trim()
        .split('\n').map(|line| line.split_whitespace()
            .map(|word| word.parse().unwrap()).collect())
        .collect();
    testcases.remove(testcases.len() - 1);
    let mut result: Vec<&str> = Vec::new();
    for t in testcases {
        let biggest_val = t.iter().max().unwrap();
        if 2 * biggest_val.pow(2) == t[0].pow(2) + t[1].pow(2) + t[2].pow(2) {
            result.push("right");
        } else {
            result.push("wrong");
        }
    }
    println!("{}", result.join("\n"));
}
