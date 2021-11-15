// baekjoon 11050 이항 계수 1
fn main() {
    
    let (a, b) = {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.trim().split_whitespace()
            .map(|x| x.parse::<usize>().unwrap());
        let a = iter.next().unwrap();
        let b = iter.next().unwrap(); 
        if a < b { (b, a) }
        else { (a, b) }
    };
    let mut a_acc = 1;
    let mut b_acc = 1;
    for _b in 0..b {
        a_acc *= a - _b;
        b_acc *= _b + 1;
    }
    println!("{}", a_acc / b_acc);
}