use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

fn main() {
    let f = read_to_string("./input.txt").unwrap();

    let f = f.split("\n\n").collect::<Vec<&str>>();
    let rules = f[0].split('\n').collect::<Vec<&str>>();
    let pages_queue = f[1].split('\n').collect::<Vec<&str>>();

    let mut rules_map_dont_come_after: HashMap<usize, Vec<usize>> = HashMap::new();
    for rule in rules {
        let rule = rule
            .split("|")
            .map(|r| r.parse().unwrap())
            .collect::<Vec<usize>>();
        if let Some(pages) = rules_map_dont_come_after.get_mut(&rule[1]) {
            pages.push(rule[0]);
        } else {
            rules_map_dont_come_after.insert(rule[1], vec![rule[0]]);
        }
    }

    let mut ans = 0;

    for pages in pages_queue {
        if pages == "" {
            continue;
        }

        let mut pages = pages
            .split(',')
            .map(|p| {
                return p.parse().unwrap();
            })
            .collect::<Vec<usize>>();

        let mut was_false = false;
        loop {
            let mut right_sequence = true;
            for now_page_i in 0..pages.len() {
                for prev_page_i in 0..now_page_i {
                    if let Some(forbidden_pages) =
                        rules_map_dont_come_after.get(&pages[prev_page_i])
                    {
                        if forbidden_pages.contains(&pages[now_page_i]) {
                            was_false = true;
                            right_sequence = false;

                            let temp_page = pages[now_page_i];
                            pages[now_page_i] = pages[prev_page_i];
                            pages[prev_page_i] = temp_page;

                            break;
                        }
                    }
                }
                if !right_sequence {
                    break;
                }
            }

            if right_sequence && !was_false {
                break;
            } else if right_sequence && was_false {
                ans += pages[pages.len() / 2];
                break;
            }
        }
    }

    println!("{}", ans);
}
