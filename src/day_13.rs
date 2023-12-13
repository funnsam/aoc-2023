fn solve(rows: &Vec<u32>, col_c: usize, orig: Option<usize>) -> Option<usize> {
    // match vert reflection
    let get_col = |idx: usize| {
        rows.iter().fold(0, |a, e| (a << 1) | ((*e >> (col_c-idx-1)) & 1))
    };

    'vert: for i in 1..col_c {
        for j in 1..i+1 {
            if i+j-1 >= col_c {
                break;
            }

            if get_col(i+j-1) != get_col(i-j) {
                continue 'vert;
            }
        }

        if let Some(orig) = orig {
            if i != orig {
                return Some(i);
            }
        } else {
            return Some(i);
        }
    }

    // match horiz reflection
    'horiz: for i in 1..rows.len() {
        for j in 1..i+1 {
            if i+j-1 >= rows.len() {
                break;
            }

            if rows[i+j-1] != rows[i-j] {
                continue 'horiz;
            }
        }

        if let Some(orig) = orig {
            if i != orig * 100 {
                return Some(i * 100);
            }
        } else {
            return Some(i * 100);
        }
    }

    None
}

pub fn part_1(file: &str) -> String {
    let mut total = 0;

    for map in file.split("\n\n") {
        let rows = map.lines().map(|a| a.as_bytes().iter().fold(0, |a, e| (a << 1) | (*e == b'#') as u32)).collect::<Vec<u32>>();
        let col_c = map.lines().next().unwrap().len();

        total += solve(&rows, col_c, None).unwrap();
    }

    total.to_string()
}

pub fn part_2(file: &str) -> String {
    println!("This doesn't work btw");

    let mut total = 0;

    'maps: for map in file.split("\n\n") {
        let rows = map.lines().map(|a| a.as_bytes().iter().fold(0, |a, e| (a << 1) | (*e == b'#') as u32)).collect::<Vec<u32>>();
        let col_c = map.lines().next().unwrap().len();

        let orig = solve(&rows, col_c, None).unwrap();
        // smudge stuff and try until it works
        for y in 0..rows.len() {
            for x in 0..col_c {
                let mut rows = rows.clone();
                rows[y] ^= 1 << (col_c - x - 1);
                if let Some(s) = solve(&rows, col_c, Some(orig)) {
                    if s != orig {
                        total += s;
                        continue 'maps;
                    }
                }
            }
        }

        println!("h");
    }

    total.to_string()
}
