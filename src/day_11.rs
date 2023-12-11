pub fn task_1(file: &str) -> String {
    let mut g = file.lines().map(|a| a.as_bytes().to_vec()).collect::<Vec<Vec<u8>>>();

    // expand y axis
    let mut e = Vec::new();
    'expand: for (yi, y) in g.iter().enumerate() {
        for x in y.iter() { if *x != b'.' { continue 'expand; } }
        e.push(yi);
    }

    for (ei, e) in e.iter().enumerate() {
        g.insert(ei+e, vec![b'.'; g[0].len()]);
    }

    // expand x axis
    let mut e = Vec::new();
    'expand: for xi in 0..g[0].len() {
        for y in 0..g.len() { if g[y][xi] != b'.' { continue 'expand; } }
        e.push(xi);
    }

    for (ei, e) in e.iter().enumerate() {
        for y in g.iter_mut() {
            y.insert(ei+e, b'.');
        }
    }

    let mut galaxies = Vec::new();
    for (yi, y) in g.iter().enumerate() {
        for (xi, x) in y.iter().enumerate() {
            if *x == b'#' {
                galaxies.push((xi, yi));
            }
        }
    }

    let mut sum = 0;
    for (ai, a) in galaxies.iter().enumerate() {
        for bi in 0..ai {
            let b = &galaxies[bi];
            let mut c = *a;

            while c != *b {
                let choices = [
                    (c.0 as isize - b.0 as isize - 1, c.1 as isize - b.1 as isize    ),
                    (c.0 as isize - b.0 as isize + 1, c.1 as isize - b.1 as isize    ),
                    (c.0 as isize - b.0 as isize    , c.1 as isize - b.1 as isize - 1),
                    (c.0 as isize - b.0 as isize    , c.1 as isize - b.1 as isize + 1),
                ];
                let mut min = isize::MAX;
                let mut nth = usize::MAX;
                for (di, d) in choices.iter().enumerate() {
                    let dist = d.0*d.0+d.1*d.1;
                    if dist < min {
                        min = dist;
                        nth = di;
                    }
                }
                c = (b.0 + choices[nth].0 as usize, b.1 + choices[nth].1 as usize);

                sum += 1;
            }
        }
    }

    sum.to_string()
}

pub fn task_2(file: &str) -> String {
    todo!();
}
