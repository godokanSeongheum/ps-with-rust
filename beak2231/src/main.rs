use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: u32 = input.trim().parse().unwrap();
    for i in 1..n {
        let mut consume = i;
        let mut tmp = consume;
        while consume > 0 {
            tmp += consume % 10;
            consume /= 10;
        }
        if tmp == n {
            println!("{}", i);
            return;
        }
    }
    println!("0");
}
