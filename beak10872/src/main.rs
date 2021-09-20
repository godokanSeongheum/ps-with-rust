use std::io;
fn fac(n: u32, mut tmp:u32) -> u32 {
    if n == 0 {
        tmp
    } else {
        tmp *= n;
        fac(n - 1, tmp)
    }
}
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input: u32 = input.trim().parse().unwrap();
    println!("{}", fac(input, 1));
}
