use std::io;
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut input: u64 = input.trim().parse().unwrap();
    
    let mut result: Vec<String> = Vec::new();

    let mut div = 2;

    while input != 1 && div <= input {
        if input % div == 0 {
            input /= div;
            result.push(div.to_string());
            continue;
        }
        div += 1;
    }
    println!("{}", result.join("\n"));   
}
