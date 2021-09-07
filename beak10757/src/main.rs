use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let v: Vec<&str> = input.trim().split_whitespace().collect();
    
    let a: Vec<u32> = v[0].chars()
        .map(|c| c.to_digit(10).unwrap())
        .rev().collect();
    let b: Vec<u32> = v[1].chars()
        .map(|c| c.to_digit(10).unwrap())
        .rev().collect();
    let c_len = if a.len() > b.len() {
        a.len() + 1
    } else {
        b.len() + 1
    };
    let mut c: Vec<u32> = vec![0;c_len];
    for i in 0..c_len - 1 {
        if i < a.len() {
            c[i] += a[i];
        }
        if i < b.len() {
            c[i] += b[i];
        }
        if c[i] >= 10 {
            c[i] -= 10;
            c[i + 1] += 1;
        }
    }
    let mut sum: Vec<String> = c.iter()
        .map(|x| x.to_string())
        .rev().collect();
    if sum.len() > 1 && sum[0] == "0" {
        sum.remove(0);
    }
    println!("{}", sum.join(""));
}
