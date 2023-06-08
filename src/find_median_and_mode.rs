use std::collections::HashMap;

pub fn get_median_and_mode(input: &mut Vec<u32>) -> (u32, u32) {
    return (find_median(input), find_mode(input));
}

fn find_median(input: &mut Vec<u32>) -> u32 {
    input.sort();

    match input.get(input.len() / 2 - 1) {
        None => panic!("Something terribly went wrong"),
        Some(val) => *val,
    }
}

fn find_mode(input: &Vec<u32>) -> u32 {
    let mut seen_count = HashMap::new();

    for i in input {
        let count = seen_count.entry(i).or_insert(0);
        *count += 1;
    }

    let max_count = seen_count
        .values()
        .max()
        .expect("Something went terribly wrong");

    return seen_count
        .iter()
        .find_map(|(&k, v)| if v == max_count { Some(*k) } else { None })
        .expect("Something went terribly wrong");
}
