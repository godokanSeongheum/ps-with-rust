// baekjoon 1541 읽어버린 괄호
use std::io;
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let chunks: Vec<&str> = input.trim().split("-").collect();
    let input: Vec<Vec<isize>> = chunks.iter()
        .map(|x| x.split("+")
            .map(|x| x.parse().unwrap()).collect()
        ).collect();
    let result: isize = input[0].iter().sum::<isize>() -
        input[1..].iter().map(|chunk| chunk.iter().sum::<isize>()).sum::<isize>();
    print!("{}", result);
}