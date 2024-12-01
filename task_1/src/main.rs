use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let f = read_to_string("./input.txt").unwrap();
    let mut left_side = vec![];
    let mut right_side = vec![];
    let mut right_side_appierance: HashMap<i32, i32> = HashMap::new();

    for line in f.lines() {
        let lr = line.split("   ").collect::<Vec<&str>>();
        left_side.push(lr[0].parse::<i32>().unwrap());
        right_side.push(lr[1].parse::<i32>().unwrap());
        if let Some(num) = right_side_appierance.get_mut(&lr[1].parse::<i32>().unwrap()) {
            *num += 1;
        } else {
            right_side_appierance.insert(lr[1].parse::<i32>().unwrap(), 1);
        }
    }
    left_side.sort();
    right_side.sort();

    let mut ans = 0;
    for i in 0..left_side.len() {
        ans += (left_side[i] - right_side[i]).abs();
    }
    println!("task 1: {}", ans);

    let mut ans = 0;
    for num in left_side.iter() {
        ans += if let Some(count) = right_side_appierance.get(&num) {
            num * count
        } else {
            0
        }
    }
    println!("task 2: {}", ans);
}
