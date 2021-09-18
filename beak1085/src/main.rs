use std::io;
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let inputs: Vec<i32> = input.trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let outputs: Vec<i32> = vec![
        inputs[2] - inputs[0],
        inputs[3] - inputs[1],
        inputs[1],
        inputs[0],
    ];
    println!("{}", outputs.iter().min().unwrap());
}
