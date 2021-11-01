// beakjoon 9184 신나는 함수 실행
use std::{
    io::Read,
    str::from_utf8,
    collections::HashMap
};

type TestCase = (isize, isize, isize);

fn resolve_testcase(testcase: &[isize]) -> TestCase {
    (testcase[0], testcase[1], testcase[2])
}

fn create_key(testcase: TestCase) -> String {
    format!("{} {} {}", testcase.0, testcase.1, testcase.2)
}

fn get_answer(testcase: TestCase, memo: &mut HashMap<String, isize>) -> isize {
    let key = create_key(testcase);
    match memo.get(&key) {
        Some(&n) => n,
        None => {
            let answer = solve(testcase, memo);
            memo.insert(key, answer);
            answer
        },
    }
}

fn solve(testcase: TestCase, memo: &mut HashMap<String, isize>) -> isize {
    let (a, b, c) = testcase;
    if a <= 0 || b <= 0 || c <= 0 {
        return 1;
    }
    if a > 20 || b > 20 || c > 20 {
        return get_answer((20, 20, 20), memo);
    }
    if a < b && b < c{
        return get_answer((a, b, c - 1), memo) + 
            get_answer((a, b - 1, c - 1), memo) -
            get_answer((a, b - 1, c), memo);
    } 
    return get_answer((a - 1, b, c), memo) + 
        get_answer((a - 1, b - 1, c), memo) +
        get_answer((a - 1, b, c - 1), memo) -
        get_answer((a - 1, b - 1, c - 1), memo);
}
fn main() {
    use std::io;
    let inputs: Vec<isize> = {
        let mut input = Vec::new();
        io::stdin().read_to_end(&mut input).unwrap();
        from_utf8(&input).unwrap().trim().split_whitespace()
            .map(|x| x.parse().unwrap()).collect()
    };
    let mut memo: HashMap<String, isize> = HashMap::new();
    for testcase in inputs[..inputs.len() - 3].chunks(3) {
        let (a, b, c) = resolve_testcase(&testcase);
        let answer = match memo.get(create_key((a, b, c)).as_str()) {
            Some(&n) => n,
            None => solve((a, b, c), &mut memo),
        };
        println!("w({}, {}, {}) = {}", a, b, c, answer);
    }
}