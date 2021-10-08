use std::io;


fn permut(arr: Vec<usize>, m: usize, prev: Vec<usize>) -> Vec<usize> {
    if m == 0 {
        return prev;
    }
    let mut output = Vec::new();
    for idx in 0..arr.len() {
        let mut tmp_arr = arr.clone();
        let val = tmp_arr.remove(idx);
        let mut cloned_prev = prev.clone();
        cloned_prev.push(val);
        output.append(&mut permut(tmp_arr, m - 1, cloned_prev)); 
    }
    return output;
}
fn main() {
    let nm: Vec<usize> = {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().split_whitespace().map(|x| x.parse().unwrap())
            .collect()
    };
    let arr: Vec<usize> = (1..=nm[0]).collect();
    let m = nm[1];
    let result = {
        let nums = permut(arr, m, Vec::new());
        let strings: Vec<String> = nums.iter().map(|x| x.to_string()).collect();
        let mut output = Vec::new();
        for s in strings.chunks(m) {
            output.push(s.join(" "));
        }
        output.join("\n")
    };
    println!("{}", result);
}
