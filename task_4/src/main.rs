use std::fs::read_to_string;

fn main() {
    let f = read_to_string("./input.txt").unwrap();
    let lines = f.split('\n').collect::<Vec<&str>>();

    // 140 is the size of the map
    let mut xmas_map = [[' '; 140]; 140];
    let mut a_coords = vec![];

    for (y, line) in lines.iter().enumerate() {
        for (x, sym) in line.chars().enumerate() {
            if sym == 'A' {
                a_coords.push((x, y));
            }
            xmas_map[y][x] = sym;
        }
    }

    let diag_dirs = vec![(-1, -1), (1, 1), (1, -1), (-1, 1)];

    let mut ans = 0;
    for (x, y) in a_coords {
        let mut diags_ch = String::from("");
        let mut diag_found = true;
        for (dir_x, dir_y) in diag_dirs.iter() {
            let new_x = x as i32 + dir_x;
            let new_y = y as i32 + dir_y;

            if new_x >= 0 && new_y >= 0 && new_x < 140 && new_y < 140 {
            } else {
                diag_found = false;
                break;
            }
            diags_ch.push(xmas_map[new_y as usize][new_x as usize]);
        }

        if diag_found
            && (diags_ch == "MSMS"
                || diags_ch == "SMMS"
                || diags_ch == "MSSM"
                || diags_ch == "SMSM")
        {
            ans += 1;
        }
    }

    println!("Answer: {}", ans);
}
