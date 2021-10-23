use std::{io::{self, Read}, str::from_utf8};
use std::fmt::Debug;
// beakjoon 9663 N-Queen
#[derive(Debug)]
struct Memo {
    map: [usize;81],
    columns: [[bool;9];9],
    rows: [[bool;9];9],
    blocks: [[bool;9];9],
}

fn get_block_idx(i: usize) -> usize { 
    let block_start_idx = i - (i / 9 % 3) * 9 - i % 3;
    block_start_idx / 27 * 3 + block_start_idx % 9 / 3
}

fn initialize_memo(memo: &mut Memo) -> () {
    for i in 0..81 {
        if memo.map[i] == 0 { continue; }
        memo.columns[i % 9][memo.map[i] - 1] = false;
        memo.rows[i / 9][memo.map[i] - 1] = false;
        memo.blocks[get_block_idx(i)][memo.map[i] - 1] = false;
    }
}

fn create_output(map: &[usize;81]) -> String {
    let mut output = String::new();
    for chunk in map.chunks(9) {
        output.push_str(
            &(chunk.iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>().join(" "))[..]
        );
        output.push('\n');
    }
    return output;
}

fn solve(memo: &mut Memo, progress: usize) -> Result<[usize;81], &str> {
    let mut i = 81;
    for k in progress..memo.map.len() {
        if memo.map[k] == 0 {
            i = k;
            break;
        }
    }
    if i == 81 {
        return Ok(memo.map);
    }
    let col_idx = i % 9;
    let row_idx = i / 9;
    let block_idx = get_block_idx(i);
    let candies: Vec<usize> = (1..10).filter(
        |x| memo.columns[col_idx][x - 1] && memo.rows[row_idx][x - 1] && memo.blocks[block_idx][x - 1]
    ).collect();
    if candies.len() == 0 {
        return Err("Nope!");
    }
    for candi in candies {
        memo.map[i] = candi;
        memo.columns[col_idx][candi - 1] = false;
        memo.rows[row_idx][candi - 1] = false;
        memo.blocks[block_idx][candi - 1] = false;
        match solve(memo, i + 1) {
            Ok(answer) => return Ok(answer),
            Err(_) => (),
        }
        memo.map[i] = 0;
        memo.columns[col_idx][candi - 1] = true;
        memo.rows[row_idx][candi - 1] = true;
        memo.blocks[block_idx][candi - 1] = true;
    }
    return Err("Done!");
}
fn main() {
    let map: Vec<usize> = {
        let mut input = Vec::new();
        io::stdin().read_to_end(&mut input).unwrap();
        from_utf8(&input).unwrap().trim().split_whitespace()
            .map(|x| x.parse().unwrap()).collect()
    };
    let map = {
        let mut arr = [0;81];
        for i in 0..81 {
            arr[i] = map[i];
        }
        arr
    };
    let mut memo = Memo { map, columns: [[true;9];9], rows: [[true;9];9], blocks: [[true;9];9] };
    initialize_memo(&mut memo);
    let answer = match solve(&mut memo, 0) {
        Ok(answer) => answer,
        Err(_) => [0;81],
    };
    
    println!("{}", create_output(&answer));
}