use std::{io::{ self, Read }, str::from_utf8};
use std::collections::HashMap;

fn main() {
    let mut elements: Vec<isize> = {
        let mut input =Vec::new();
        io::stdin().read_to_end(&mut input).unwrap();
        from_utf8(&input).unwrap().trim().split_whitespace()
            .map(|x| x.parse().unwrap()).skip(1).collect()
    };
    let mut result: [isize;4] = [0;4];
    result[0] = (elements.iter().sum::<isize>() as f64 / elements.len() as isize as f64).round() as isize;

    elements.sort();
    result[1] = elements[(elements.len() - 1) / 2];
    
    let mut hm: HashMap<isize, usize> = HashMap::new();
    for x in &elements {
        *hm.entry(*x).or_insert(0) += 1;
    }
    let max_val = *(hm.iter()
        .max_by_key(|(_, &v)| v).unwrap().1);
    let mut candidates: Vec<(&isize, &usize)> = hm.iter().filter(|(_, &v)| v == max_val).collect();
    candidates.sort_by_key(|&(&k, _)| k);
    result[2] = if candidates.len() == 1 {
        *(candidates[0].0)
    } else {
        *(candidates[1].0)
    };

    result[3] = elements[elements.len() - 1] - elements[0];
    
    for r in result.iter() {
        println!("{}", r);
    }
}
