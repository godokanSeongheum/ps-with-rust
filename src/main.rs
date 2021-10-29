use std::{io::{self, Read}, str::from_utf8};
// beakjoon 14889 스타트와 링크

fn set_min_val(tmp: usize, min_val: &mut usize, other: usize) {
    let difference = (other  as isize - tmp as isize).abs();
    if  difference < *min_val as isize {
        *min_val = difference as usize;
    }
    return;
}

fn join_team(idx: usize, synergies: &Vec<Vec<usize>>, tmp: usize, plate: &mut Vec<usize>) -> usize {
    let mut new_tmp = tmp;
    for &teammate in plate.iter() {
        new_tmp = new_tmp + synergies[idx][teammate] + synergies[teammate][idx];
    }
    plate.push(idx);
    new_tmp
}

fn get_opposite_team_score(synergies: &Vec<Vec<usize>>, plate: &Vec<usize>) -> usize {
    let mut score = 0;
    for i in 0..synergies.len() - 1 {
        if plate.contains(&i) {
            continue;
        }
        for j in i + 1..synergies.len() {
            if plate.contains(&j) {
                continue;
            }
            score = score + synergies[i][j] + synergies[j][i];
        }
    }
    score
}
fn solve(
    min_val: &mut usize,
    left: usize,
    synergies: &Vec<Vec<usize>>,
    tmp: usize,
    cur_idx: usize,
    plate: &mut Vec<usize>) {
    if left == 0 {
        let other = get_opposite_team_score(synergies, plate);
        set_min_val(tmp, min_val, other);
        return;
    }
    let mut tmp = tmp;
    if synergies.len() as isize - cur_idx as isize == left as isize {
        for idx in cur_idx..synergies.len() {
            tmp = join_team(idx, synergies, tmp, plate);
        }
        let other = get_opposite_team_score(synergies, plate);
        set_min_val(tmp, min_val, other);
        for _ in cur_idx..synergies.len() {
            plate.pop();
        }
        return;
    }

    for i in cur_idx..synergies.len() {
        let new_tmp = join_team(i, synergies, tmp, plate);
        solve(min_val, left - 1, synergies, new_tmp, i + 1, plate);
        plate.pop();
    }
    return;
}

fn main() {
    let (n, synergies): (usize, Vec<Vec<usize>>) = {
        let mut stdinput = Vec::new();
        io::stdin().read_to_end(&mut stdinput).unwrap();
        let stdinput: Vec<usize> = from_utf8(&stdinput).unwrap().trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap()).collect();
        let mut synergies = vec![];
        for chunk in stdinput[1..].chunks(stdinput[0]) {
            synergies.push(chunk.to_vec());
        }
        (stdinput[0], synergies)
    };

    let mut min_val = isize::MAX as usize;
    let mut plate: Vec<usize> = Vec::new();
    solve(&mut min_val, n / 2, &synergies, 0, 0, &mut plate);
    println!("{}", min_val);
}