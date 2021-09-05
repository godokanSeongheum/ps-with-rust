use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: u32 = input.trim().parse().unwrap();
    input.clear();
    for _ in 0..t {
        io::stdin().read_line(&mut input).unwrap();
        let k: usize = input.trim().parse().unwrap();
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let u: usize = input.trim().parse().unwrap();
        input.clear();
        let mut v: Vec<Vec<usize>> = vec![vec![0;u];k];
        for i in 0..k {
            v[i][0] = 1;
        }
        for i in 1..u {
            v[0][i] = v[0][i-1] + i + 1;
        }
        for i in 1..k {
            for j in 1..u {
                v[i][j] = v[i-1][j] + v[i][j-1];
            }
        } 
        println!("{}", v[k-1][u-1]);
    }
}
