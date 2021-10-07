use std::{io::{self, Read}, str::from_utf8};

fn main() {
    let mut words: Vec<String> = {
        let mut input: Vec<u8> = Vec::new();
        io::stdin().read_to_end(&mut input).unwrap();
        from_utf8(&input).unwrap().trim().split_whitespace()
            .skip(1).map(|x| x.to_string()).collect()
    };
    words.sort();
    words.dedup();
    words.sort_by_key(|word| word.len());

    println!("{}", words.join("\n"));
}