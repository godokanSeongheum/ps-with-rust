use std::{io::{self, Read}, str::from_utf8};
// beakjoon 14888 연산자 끼워넣기

struct Answer {
    max: isize,
    min: isize,
}

fn operate(code: usize, prev: isize, cur: isize) -> isize {
    match code {
        0 => prev + cur,
        1 => prev - cur,
        2 => prev * cur,
        3 => prev / cur,
        _ => -1,
    }
}

fn compose(nums: &Vec<isize>, operators: &mut Vec<isize>, prev: isize, idx: usize, answer: &mut Answer) {
    if idx == nums.len() {
        answer.max = prev.max(answer.max);
        answer.min = prev.min(answer.min);
        return;
    }
    let cur = nums[idx];
    for code in 0..4 {
        if operators[code] == 0 {
            continue;
        }
        operators[code] -= 1;
        let next = operate(code, prev, cur);
        compose(nums, operators, next, idx + 1, answer);
        operators[code] += 1;
    }
}
fn main() {
    let (nums, mut operators): (Vec<isize>, Vec<isize>) = {
        let mut input = Vec::new();
        io::stdin().read_to_end(&mut input).unwrap();
        let mut input: Vec<isize> = from_utf8(&input).unwrap().trim().split_whitespace()
            .skip(1).map(|x| x.parse().unwrap()).collect();
        let operators: Vec<isize> = input.drain(input.len() - 4..input.len()).collect();
        (input, operators)
    };
    let mut answer = Answer {max: (-10 as isize).pow(9), min: (10 as isize).pow(9)};
    compose(&nums, &mut operators, nums[0], 1, &mut answer);
    println!("{}\n{}", answer.max, answer.min);
}