use std::io;
use std::f64::consts::PI;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let r: f64 = input.trim().parse().unwrap();
    println!("{}", PI * r.powf(2.0));
    println!("{}", 2.0 * r.powf(2.0));
}
