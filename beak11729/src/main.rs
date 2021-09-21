use std::io;

struct HanoiEnv {
    count: u32,
    progress: Vec<String>,
}
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: u32 = input.trim().parse().unwrap();
    let mut hanoi_env = HanoiEnv { count: 0, progress: Vec::new() };
    let src = 1;
    let waypoint = 2;
    let dest = 3;
    fn hanoi(n: u32, s: u32, d: u32, w: u32, hanoi_env: &mut HanoiEnv) -> ()  {
        if n == 0 {
            return;
        }
        hanoi(n - 1, s, w, d, hanoi_env);
        hanoi_env.count += 1;
        hanoi_env.progress.push([s.to_string(), d.to_string()].join(" "));
        hanoi(n - 1, w, d, s, hanoi_env);
    }
    hanoi(n, src, dest, waypoint, &mut hanoi_env);
    println!("{}", hanoi_env.count);
    println!("{}", hanoi_env.progress.join("\n"));
}
