use std::io;
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let v: Vec<usize> = input.trim().split_whitespace()
        .map(|x| x.parse().unwrap()).collect();
    let mut result: Vec<String> = Vec::new();
    let mut all: Vec<bool> = vec![true;v[1] + 1];
    all[1] = false;
    for n in 2..=v[1] {
        if all[n] {
            if n >= v[0] {
                result.push(n.to_string());
            }
            let mut i = 2;
            while i * n <= v[1] {
                all[i * n] = false;
                i += 1;
            }
        }
    }
    println!("{}", result.join("\n"));
}
