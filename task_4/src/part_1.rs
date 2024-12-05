use std::fs::read_to_string;

fn main() {
    let f = read_to_string("./input.txt").unwrap();
    let lines = f.split('\n').collect::<Vec<&str>>();

    // 140 is the size of the map
    let mut xmas_map = [[' '; 140]; 140];
    let mut x_coords = vec![];

    for (y, line) in lines.iter().enumerate() {
        for (x, sym) in line.chars().enumerate() {
            if sym == 'X' {
                x_coords.push((x, y));
            }
            xmas_map[y][x] = sym;
        }
    }

    let dirs = vec![
        (0, -1),
        (0, 1),
        (-1, 0),
        (1, 0),
        (-1, -1),
        (1, -1),
        (-1, 1),
        (1, 1),
    ];

    let mut ans = 0;
    for (x, y) in x_coords {
        for (dir_x, dir_y) in dirs.iter() {
            let mut xmas_found = true;

            for (i, xmas_ch) in vec!['X', 'M', 'A', 'S'].iter().enumerate() {
                let new_x = x as i32 + dir_x * i as i32;
                let new_y = y as i32 + dir_y * i as i32;

                if new_x >= 0 && new_y >= 0 && new_x < 140 && new_y < 140 {
                    if xmas_map[new_y as usize][new_x as usize] != *xmas_ch {
                        xmas_found = false;
                        break;
                    }
                } else {
                    xmas_found = false;
                    break;
                }
            }

            if xmas_found {
                ans += 1;
            }
        }
    }

    println!("Answer: {}", ans);
}
