use std::io;
// beakjoon 9663 N-Queen

fn main() {
    let n: usize = {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().parse().unwrap()
    };
    let map: Vec<isize> = vec![-1; n];
    fn get_queens(map: &Vec<isize>, depth: usize) -> usize {
        let n = map.len();
        if depth == n {
            return if map[n - 1] == -1 as isize {0} else {1};
        }
        let mut answer = 0;
        for i in 0..n {
            let mut good_to_go = true;
            for j in 0..depth {
                if map[j] == i as isize {
                    good_to_go = false;    // vertical
                    break;
                }
                if (depth - j) as isize == (map[j] - i as isize).abs() { // diagonal
                    good_to_go = false;
                    break;
                }
            }
            if good_to_go {
                let mut new_branch = map.clone();
                new_branch[depth] = i as isize;
                answer += get_queens(&new_branch, depth + 1);
            }
        }
        return answer;
    }
    println!("{}", get_queens(&map, 0));
}