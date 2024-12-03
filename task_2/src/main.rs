use std::{collections, fs::read_to_string};

fn main() {
    let f = read_to_string("./input.txt").unwrap();

    let mut safe_roads = 0;
    for line in f.lines() {
        let nums = line
            .split(' ')
            .into_iter()
            .map(|num| num.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        if check_nums(&nums, false) {
            safe_roads += 1;
        }
    }

    print!("{safe_roads}");
}

fn check_nums(nums: &Vec<i32>, unsafe_found: bool) -> bool {
    let mut descending = false;
    let mut was_checked = false;
    for i in 0..nums.len() - 1 {
        if !was_checked && nums[i] != nums[i + 1] {
            descending = nums[i] > nums[i + 1];
            was_checked = true;
        }

        if check_places_unsafety(nums[i], nums[i + 1], descending, was_checked) {
            if !unsafe_found {
                let mut nums1 = nums.clone();
                let mut nums2 = nums.clone();
                let mut nums3 = nums.clone();
                // if the one of both levels are unsafe
                nums1.remove(i);
                nums2.remove(i + 1);
                // if the descending was not properly defined
                nums3.remove(0);

                // if one of this levels combo are safe, then return safe
                return check_nums(&nums1, true)
                    || check_nums(&nums2, true)
                    || check_nums(&nums3, true);
            } else {
                return false;
            }
        }
    }

    true
}

fn check_places_unsafety(num1: i32, num2: i32, descending: bool, was_checked: bool) -> bool {
    return (num1 - num2).abs() > 3
        || (num1 - num2).abs() < 1
        || was_checked && descending && !(num1 > num2)
        || was_checked && !descending && !(num1 < num2);
}
