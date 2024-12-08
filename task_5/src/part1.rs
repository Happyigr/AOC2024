use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

fn main() {
    let f = read_to_string("./input.txt").unwrap();

    let f = f.split("\n\n").collect::<Vec<&str>>();
    let rules = f[0].split('\n').collect::<Vec<&str>>();
    let pages_queue = f[1].split('\n').collect::<Vec<&str>>();

    let mut rules_map: HashMap<usize, Vec<usize>> = HashMap::new();
    for rule in rules {
        let rule = rule
            .split("|")
            .map(|r| r.parse().unwrap())
            .collect::<Vec<usize>>();
        if let Some(pages) = rules_map.get_mut(&rule[1]) {
            pages.push(rule[0]);
        } else {
            rules_map.insert(rule[1], vec![rule[0]]);
        }
    }

    let mut ans = 0;

    for pages in pages_queue {
        if pages == "" {
            continue;
        }
        let pages = pages
            .split(',')
            .map(|p| {
                return p.parse().unwrap();
            })
            .collect::<Vec<usize>>();

        let mut forbidden_pages: HashSet<usize> = HashSet::new();
        let mut right_sequence = true;
        for page in pages.iter() {
            if forbidden_pages.contains(&page) {
                right_sequence = false;
                break;
            }
            if let Some(pages) = rules_map.get(&page) {
                forbidden_pages.extend(pages);
            }
        }

        if right_sequence {
            let pages_amount = pages.len();
            ans += pages[pages_amount / 2]
        }
    }

    println!("{}", ans);
}
