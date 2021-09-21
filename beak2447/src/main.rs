use std::io;
fn punch(mut picture: Vec<Vec<char>>, n: usize) -> Vec<Vec<char>> {
    if n == 1 {
        return picture;
    }
    let mut c = 0;
    while c * n + n / 3 * 2 <= picture[0].len() {
        let mut r = 0;
        while r * n + n / 3 * 2 <= picture[0].len() {
            let col_range = c * n + n / 3..c * n + n / 3 * 2;
            let row_range = r * n + n / 3..r * n + n / 3 * 2;
            for ci in col_range {
                for ri in row_range.clone() {
                    picture[ri][ci] = ' ';
                }
            }
            r += 1;
        }
        c += 1;
    }
    return punch(picture, n / 3);
}
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();
    let picture: Vec<Vec<char>> = vec![vec!['*';n];n];
    let v: String = punch(picture, n).iter()
        .map(|row| -> String { row.iter().collect() })
        .collect::<Vec<String>>().join("\n");
    println!("{}",v);
}
