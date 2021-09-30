use std::io;
fn main() {
    let n: usize = {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().parse().unwrap()
    };
    let mut count = 0;
    let mut num: usize = 665;
    while n > count {
        loop {
            num += 1;
            if num.to_string().contains("666") {
                break;
            }
        }
        count += 1;
    }
    println!("{}", num);
}
