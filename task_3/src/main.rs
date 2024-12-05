use std::fs::read_to_string;

use regex::Regex;

fn main() {
    let mut f = read_to_string("./found").unwrap();

    let mut ans = 0;
    for mul in find_mul(&f) {
        ans += exec_mul(mul);
    }

    // with this regex i found all of the matches, and then in the matches i used the functions
    // from the first part.
    //
    // r"(don't\(\))(?:[^d]|d(?!o\(\)))*?(do\(\))|(don't\(\))|(do\(\))";
    //

    println!("{}", 170778545 - ans);
}

fn exec_mul(mul_str: &str) -> usize {
    let pattern = r"\d+";
    let re = Regex::new(pattern).unwrap();
    let nums = re
        .find_iter(mul_str)
        .map(|num| num.as_str().parse().unwrap())
        .collect::<Vec<usize>>();
    return nums[0] * nums[1];
}

fn find_mul(s: &str) -> Vec<&str> {
    let pattern = r"mul\(\d+,\d+\)";
    let re = Regex::new(pattern).unwrap();

    let matches = re.find_iter(s).map(|m| m.as_str()).collect::<Vec<&str>>();
    return matches;
}

// part2 first try

// let pattern = r"don't\(\)";
// let dont_re = Regex::new(pattern).unwrap();
// let pattern = r"do\(\)";
// let do_re = Regex::new(pattern).unwrap();
// // this pattern finds all between do() and don't() with the
// // let pattern = r"(do\(\)).*?(don't\(\))";
// // let pattern = r"(don't\(\)).*?(do\(\))";
// let pattern = r"(don't\(\))(?:[^d]|d(?!o\(\)))*?(do\(\))|(don't\(\))|(do\(\))";
// let between_re = Regex::new(pattern).unwrap();
//
// let mut ans = 0;
//
// let mut last_founded_dont = 0;
//
// // if let Some(dont_i) = dont_re.find(&f) {
// //     last_founded_dont = dont_i.range().last().unwrap();
// //     for mul in find_mul(&f[0..last_founded_dont]) {
// //         ans += exec_mul(mul);
// //     }
// // }
// for between in between_re.find_iter(&f) {
//     for mul in find_mul(between.as_str()) {
//         ans += exec_mul(mul);
//     }
//     last_founded_dont = between.range().last().unwrap();
// }
//
// // let temp = &f[last_founded_dont..];
// // if let Some(do_i) = do_re.find(temp) {
// //     for mul in find_mul(&temp[do_i.range().start..]) {
// //         ans += exec_mul(mul);
// //     }
// // }
//
