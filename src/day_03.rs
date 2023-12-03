pub fn task_1(file: String) {
    let mut sum = 0;

    let file = file.lines().map(|a| a.chars().collect()).collect::<Vec<Vec<char>>>();

    for (yi, y) in file.iter().enumerate() {
        let mut x = y.iter().enumerate();
        'a: while let Some((xi, i)) = x.next() {
            if i.is_ascii_digit() {
                let mut end = xi;
                for (_, j) in x.by_ref() {
                    if j.is_ascii_digit() {
                        end += 1;
                    } else {
                        break;
                    }
                }
                macro_rules! put {
                    ($i: expr) => {
                        let i = $i;
                        if *i != '.' && !i.is_ascii_digit() {
                            sum += (file[yi][xi..=end].to_vec()).iter().collect::<String>().parse::<usize>().unwrap();
                            continue 'a;
                        }
                    };
                }
                if yi > 0 {
                    for i in file[yi-1].iter().skip(xi.saturating_sub(1)).take(end - xi + (xi != 0) as usize + 2) {
                        put!(i);
                    }
                }
                if yi+1 < file.len() {
                    for i in file[yi+1].iter().skip(xi.saturating_sub(1)).take(end - xi + (xi != 0) as usize + 2) {
                        put!(i);
                    }
                }
                if xi > 0 {
                    put!(&y[xi-1]);
                }
                if end+1 < y.len() {
                    put!(&y[end+1]);
                }
            }
        }
    }

    report!("{sum}");
}

pub fn task_2(file: String) {
    use std::collections::HashMap;

    let mut sum = 0;

    let mut gears = HashMap::new();

    let file = file.lines().map(|a| a.chars().collect()).collect::<Vec<Vec<char>>>();

    for (yi, y) in file.iter().enumerate() {
        let mut x = y.iter().enumerate();
        'a: while let Some((xi, i)) = x.next() {
            if i.is_ascii_digit() {
                let mut end = xi;
                for (_, j) in x.by_ref() {
                    if j.is_ascii_digit() {
                        end += 1;
                    } else {
                        break;
                    }
                }
                macro_rules! put {
                    ($i: expr, $p: expr) => {
                        let i = $i;
                        let p = $p;
                        if *i == '*' {
                            let num = (file[yi][xi..=end].to_vec()).iter().collect::<String>().parse::<usize>().unwrap();
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
                    for (xj, i) in file[yi-1].iter().enumerate().skip(xi.saturating_sub(1)).take(end - xi + (xi != 0) as usize + 2) {
                        put!(i, (xj, yi-1));
                    }
                }
                if yi+1 < file.len() {
                    for (xj, i) in file[yi+1].iter().enumerate().skip(xi.saturating_sub(1)).take(end - xi + (xi != 0) as usize + 2) {
                        put!(i, (xj, yi+1));
                    }
                }
                if xi > 0 {
                    put!(&y[xi-1], (xi-1, yi));
                }
                if end+1 < y.len() {
                    put!(&y[end+1], (end+1, yi));
                }
            }
        }
    }

    report!("{sum}");
}
