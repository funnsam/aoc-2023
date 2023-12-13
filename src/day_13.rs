pub fn part_1(file: &str) -> String {
    let mut total = 0;

    'maps: for map in file.split("\n\n") {
        let rows = map.lines().map(|a| a.as_bytes().iter().fold(0, |a, e| (a << 1) | (*e == b'#') as u32)).collect::<Vec<u32>>();
        let col_c = map.len() / (rows.len() + 1);

        // match horiz reflection
        for skip in 1..rows.len() {
            'horiz: for idx in 0..rows.len()-1 {
                let mut other = rows.len()-skip;
                for j in rows.iter().skip(idx) {
                    if other >= rows.len() || *j != rows[other] {
                        println!("f h {idx} {skip} {j:b} {:b} {other}", rows[other.min(rows.len()-1)]);
                        continue 'horiz;
                    }
                    other -= 1;
                    if other == usize::MAX {
                        println!("o h {idx} {skip} {j:b}");
                        break;
                    }
                }

                println!("d h {idx} {skip}");
                total += (idx + rows.len()-1).div_ceil(2) * 100;
                continue 'maps;
            }
        }

        // match vert reflection
        let get_col = |idx: usize| {
            rows.iter().fold(0, |a, e| (a << 1) | ((*e >> (col_c-idx)) & 1))
        };

        for skip in 1..col_c {
            'vert: for idx in 0..col_c {
                let mut other = col_c-skip;
                for jdx in idx..col_c {
                    if get_col(jdx) != get_col(other) {
                        println!("f v {idx} {skip} {:b} {:b} {other}", get_col(jdx), get_col(other.min(rows.len()-1)));
                        continue 'vert;
                    }

                    other -= 1;
                    if other == usize::MAX {
                        println!("o v {idx} {skip} {:b}", get_col(jdx));
                        break;
                    }
                }

                println!("d v");
                total += (idx - 1 + col_c).div_ceil(2);
                continue 'maps;
            }
        }
    }

    total.to_string()
}

pub fn part_2(file: &str) -> String {
    todo!();
}
