pub fn part_1(file: &str) -> String {
    let mut sum = 0;

    let file = file.as_bytes().chunks(141).map(|a| &a[0..140]).collect::<Vec<&[u8]>>();

    for (yi, y) in file.iter().enumerate() {
        let mut x = y.iter().enumerate();
        'a: while let Some((xi, i)) = x.next() {
            if i.is_ascii_digit() {
                let mut end = 139;
                for (i, j) in x.by_ref() {
                    if !j.is_ascii_digit() {
                        end = i-1;
                        break;
                    }
                }

                macro_rules! put {
                    ($i: expr) => {
                        let i = $i;
                        if i != b'.' && !i.is_ascii_digit() {
                            sum += unsafe { std::str::from_utf8_unchecked(&file[yi][xi..=end]) }.parse::<usize>().unwrap();
                            continue 'a;
                        }
                    };
                }
                if yi > 0 {
                    for i in file[yi-1][xi.saturating_sub(1)..(end + 2).min(140)].iter() {
                        put!(*i);
                    }
                }
                if yi+1 < file.len() {
                    for i in file[yi+1][xi.saturating_sub(1)..(end + 2).min(140)].iter() {
                        put!(*i);
                    }
                }
                if xi > 0 {
                    put!(y[xi-1]);
                }
                if end+1 < y.len() {
                    put!(y[end+1]);
                }
            }
        }
    }

    sum.to_string()
}

pub fn part_2(file: &str) -> String {
    use std::collections::HashMap;

    let mut sum = 0;

    let mut gears = HashMap::new();

    let file = file.as_bytes().chunks(141).map(|a| &a[0..140]).collect::<Vec<&[u8]>>();

    for (yi, y) in file.iter().enumerate() {
        let mut x = y.iter().enumerate();
        'a: while let Some((xi, i)) = x.next() {
            if i.is_ascii_digit() {
                let mut end = 139;
                for (i, j) in x.by_ref() {
                    if !j.is_ascii_digit() {
                        end = i-1;
                        break;
                    }
                }

                macro_rules! put {
                    ($i: expr, $p: expr) => {
                        let i = $i;
                        let p = $p;
                        if i == b'*' {
                            let num = unsafe { std::str::from_utf8_unchecked(&file[yi][xi..=end]) }.parse::<usize>().unwrap();
                            if let Some((j, g)) = gears.get(&p) {
                                if !j {
                                    sum += g * num;
                                    gears.insert(p, (true, g*num));
                                } else {
                                    sum -= g;
                                    gears.insert(p, (true, 0));
                                }
                            } else {
                                gears.insert(p, (false, num));
                            }
                            continue 'a;
                        }
                    };
                }
                if yi > 0 {
                    for (xj, i) in file[yi-1][xi.saturating_sub(1)..(end + 2).min(140)].iter().enumerate() {
                        put!(*i, (xi.saturating_sub(1)+xj, yi-1));
                    }
                }
                if yi+1 < file.len() {
                    for (xj, i) in file[yi+1][xi.saturating_sub(1)..(end + 2).min(140)].iter().enumerate() {
                        put!(*i, (xi.saturating_sub(1)+xj, yi+1));
                    }
                }
                if xi > 0 {
                    put!(y[xi-1], (xi-1, yi));
                }
                if end+1 < y.len() {
                    put!(y[end+1], (end+1, yi));
                }
            }
        }
    }

    sum.to_string()
}
