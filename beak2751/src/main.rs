use std::{io::{ self, Read}, str::from_utf8};
fn main() {
    let mut inputs: Vec<isize> = {
        let mut stdin: Vec<u8> = Vec::new();
        io::stdin().read_to_end(&mut stdin).unwrap();
        from_utf8(&stdin).unwrap().trim().split_whitespace()
            .skip(1).map(|x| x.parse().unwrap()).collect()
    };
    inputs.sort();
    let result: String = inputs.iter().map(|x| x.to_string())
        .collect::<Vec<String>>().join("\n");
    println!("{}", result);
}