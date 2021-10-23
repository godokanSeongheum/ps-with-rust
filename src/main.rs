use std::io;
// beakjoon 9663 N-Queen

fn get_queens(n: usize, depth: usize, map: &Vec<Vec<bool>>) -> usize {
    let mut answer = 0;
    if depth + 1 == n {
        return map[depth].iter().filter(|&x| *x).count();
    }
    if map[depth].iter().filter(|&x| *x).count() == 0 {
        return 0;
    }
    for i in 0..n {
        if !map[depth][i] {
            continue;
        }
        let mut new_branch = map.clone();
        // vertical
        for j in depth + 1..n {
            new_branch[j][i] = false;
        }
        // diagonals
        let lesser1 = i.min(n - depth -1);
        let lesser2 = (n - i - 1).min(n - depth - 1);
        for q in 1..=lesser1 {
            new_branch[depth + q][i - q] = false;
        }
        for q in 1..=lesser2 {
            new_branch[depth + q][i + q] = false;
        }
        answer += get_queens(n, depth + 1, &new_branch);
    }
    return answer;
}
fn main() {
    let n: usize = {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().parse().unwrap()
    };
    let map: Vec<Vec<bool>> = vec![vec![true; n]; n];
    println!("{}", get_queens(n, 0, &map));
}