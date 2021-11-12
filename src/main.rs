// baekjoon 13305 주유소
use std::io;
fn main() {
    let mut n = String::new();
    let mut distances = String::new();
    let mut prices = String::new();
    let reader  = io::stdin();
    reader.read_line(&mut n).unwrap();
    reader.read_line(&mut distances).unwrap();
    reader.read_line(&mut prices).unwrap();
    let distances: Vec<u64> = distances.trim().split_whitespace()
        .map(|x| x.parse().unwrap()).collect();
    let prices: Vec<u64> = prices.trim().split_whitespace()
        .map(|x| x.parse().unwrap()).collect();
    
    let mut budget: u64 = 0;
    let mut minimum_price: u64 = u64::MAX;
    for i in 0..distances.len() {
        minimum_price = minimum_price.min(prices[i]);
        budget += minimum_price * distances[i];
    }
    println!("{}", budget);
}