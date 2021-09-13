use std::{io::{self, Read}, str::from_utf8};
fn main() {
    let mut input: Vec<u8> = Vec::new();
    io::stdin().read_to_end(&mut input).unwrap();
    let nums: Vec<u32> = from_utf8(&input).unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap()).collect::<Vec<u32>>()
        [1..].to_vec();
    let mut area: [bool;1001] = [true;1001];
    area[1] = false;
    for i in 2..1001 {
        if area[i] {
            let mut j = 2;
            while i * j < 1001 {
                area[i * j] = false;
                j += 1;
            }
        }
    }
    
    let mut answer = 0;
    for num in nums {
        if area[num as usize] {
            answer += 1;
        }
    }
    println!("{}", answer);
}
