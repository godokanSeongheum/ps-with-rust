use std::io;
fn main() {
    let reader = io::stdin();
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let t: u32 = input.trim().parse().unwrap();
    input.clear();

    for _ in 0..t {
		reader.read_line(&mut input).unwrap();
        let v: Vec<u64> = input.trim().split_whitespace()
            .map(|c| c.parse().unwrap()).collect();
        let v = v[1] - v[0];
        let mut i: u64 = 0;
        loop {
            if v <= (i.pow(2) + i) {
                if v > (i.pow(2)) {
                    println!("{}", 2 * i); 
                } else {
                    println!("{}", 2 * i - 1);
                }
                break;
            }
            i += 1;
        }
        input.clear();
    }
    return;
}
