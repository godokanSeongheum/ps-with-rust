use std::io::{ self, Read };
use std::str::from_utf8;

fn main() {
    let mut input: Vec<u8> = Vec::new();
    io::stdin().read_to_end(&mut input).unwrap();
    let mut testcases: Vec<i64> = from_utf8(&input).unwrap().trim()
        .split_whitespace().map(|x| x.parse().unwrap()).collect();
    let mut results: Vec<&str> = Vec::new();
    testcases.remove(0);
    for t in testcases.chunks(6) {
        let powered_dist_btw_dots = (t[0] - t[3]).pow(2) + (t[1] - t[4]).pow(2);
        let powered_dist_btw_rs = (t[2] - t[5]).pow(2);
        // 중심이 같을 때
        if t[0] == t[3] && t[1] == t[4] {
            if t[2] == t[5] {
                results.push("-1");
            } else {
                results.push("0");
            }
            continue;
        }

        // 원의 중심이 서로의 외부에 있을 때
        if powered_dist_btw_dots > t[2].pow(2) && powered_dist_btw_dots > t[5].pow(2) {
            // 외부에서 닿지 않을 때
            if powered_dist_btw_dots > (t[2] + t[5]).pow(2) {
                results.push("0");
                continue;
            }
            // 외부에서 닿을 때
            if powered_dist_btw_dots == (t[2] + t[5]).pow(2) {
                results.push("1");
                continue;
            }
            results.push("2");
            continue;
        }
        // 원이 내부에 포함되어 있을 때
        if powered_dist_btw_dots < powered_dist_btw_rs {
            results.push("0");
            continue;
        }
        // 내부에서 닿을 때
        if powered_dist_btw_dots == powered_dist_btw_rs {
            results.push("1");
            continue;
        }
        results.push("2");
    }
    println!("{}", results.join("\n"));
}
