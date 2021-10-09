use std::io;
enum CustomError {
    NoAnswer,
}
fn combi(arr: Vec<usize>, m: usize, prev: Vec<usize>) -> Result<Vec<usize>, CustomError> {
    if m > arr.len() {
        return Err(CustomError::NoAnswer);
    }
    if m == 0 {
        return Ok(prev);
    }
    let mut output = Vec::new();
    for i in 0..arr.len() {
        let mut cloned_arr = arr.clone(); 
        let val = cloned_arr.remove(i);
        let mut tmp_prev = prev.clone();
        tmp_prev.push(val);
        match combi(arr.clone(), m - 1, tmp_prev) {
            Ok(mut answer) => output.append(&mut answer),
            Err(_) => (),
        }
    }
    return Ok(output);
}
fn main() {
    let nm: Vec<usize> = {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect()
    };
    let arr: Vec<usize> = (1..=nm[0]).collect();
    let m = nm[1];
    let output = match combi(arr, m, Vec::new()) {
        Ok(answer) => answer,
        Err(_) => Vec::new(),
    };
    let strings: Vec<String> = output.iter().map(|x| x.to_string()).collect();
    let mut result = String::new();
    for s in strings.chunks(m) {
        result.push_str(s.join(" ").as_str());
        result.push('\n');
    }
    println!("{}", result);
}