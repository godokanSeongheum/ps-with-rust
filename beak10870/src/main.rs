use std::io;
fn fiv(mut prev: u32, mut now: u32, n: u32) -> u32 {
    if n == 0 {
        prev
    } else if n == 1 {
        now
    } else {
        let tmp = prev + now;
        prev = now;
        now = tmp;
        fiv(prev, now, n - 1)
    }
}
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: u32 = input.trim().parse().unwrap();
    println!("{}", fiv(0, 1, n));
}
