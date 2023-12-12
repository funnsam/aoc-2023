pub fn part_1(file: &str) -> String {
    let g = file.lines().map(|a| a.as_bytes()).collect::<Vec<&[u8]>>();
    let mut s = (0, 0);

    'find_s: for (yi, y) in g.iter().enumerate() {
        for (xi, x) in y.iter().enumerate() {
            if *x == b'S' {
                s = (xi, yi);
                break 'find_s;
            }
        }
    }

    let mut g_idx = vec![vec![0; g[0].len()]; g.len()];

    fn traverse(s: (usize, usize), p: (usize, usize), i: usize, g: &Vec<&[u8]>, g_idx: &mut Vec<Vec<usize>>) {
        if p.0 < g[0].len() && p.1 < g.len() && g_idx[p.1][p.0] < i && g[p.1][p.0] != b'.' {
            g_idx[p.1][p.0] = i;
            match g[p.1][p.0] {
                b'|' => {
                    let a = (p.0, p.1-1);
                    let b = (p.0, p.1+1);
                    if a != s { traverse(p, a, i+1, g, g_idx); }
                    if b != s { traverse(p, b, i+1, g, g_idx); }
                },
                b'-' => {
                    let a = (p.0-1, p.1);
                    let b = (p.0+1, p.1);
                    if a != s { traverse(p, a, i+1, g, g_idx); }
                    if b != s { traverse(p, b, i+1, g, g_idx); }
                },
                b'L' => {
                    let a = (p.0, p.1-1);
                    let b = (p.0+1, p.1);
                    if a != s { traverse(p, a, i+1, g, g_idx); }
                    if b != s { traverse(p, b, i+1, g, g_idx); }
                },
                b'J' => {
                    let a = (p.0, p.1-1);
                    let b = (p.0-1, p.1);
                    if a != s { traverse(p, a, i+1, g, g_idx); }
                    if b != s { traverse(p, b, i+1, g, g_idx); }
                },
                b'F' => {
                    let a = (p.0, p.1+1);
                    let b = (p.0+1, p.1);
                    if a != s { traverse(p, a, i+1, g, g_idx); }
                    if b != s { traverse(p, b, i+1, g, g_idx); }
                },
                b'7' => {
                    let a = (p.0, p.1+1);
                    let b = (p.0-1, p.1);
                    if a != s { traverse(p, a, i+1, g, g_idx); }
                    if b != s { traverse(p, b, i+1, g, g_idx); }
                },
                _ => {},
            }
        }
    }

    traverse(s, (s.0-1, s.1), 1, &g, &mut g_idx);
    traverse(s, (s.0+1, s.1), 1, &g, &mut g_idx);
    traverse(s, (s.0, s.1-1), 1, &g, &mut g_idx);
    traverse(s, (s.0, s.1+1), 1, &g, &mut g_idx);

    (g_idx.iter().flatten().max().unwrap() / 2).to_string()
}

pub fn part_2(file: &str) -> String {
    let g = file.lines().map(|a| a.as_bytes()).collect::<Vec<&[u8]>>();
    let mut s = (0, 0);

    'find_s: for (yi, y) in g.iter().enumerate() {
        for (xi, x) in y.iter().enumerate() {
            if *x == b'S' {
                s = (xi, yi);
                break 'find_s;
            }
        }
    }

    let mut g_idx = vec![vec![false; g[0].len()]; g.len()];

    fn connects(c: u8) -> &'static [(isize, isize)] {
        match c {
            b'|' => &[(0, -1), (0, 1)],
            b'-' => &[(-1, 0), (1, 0)],
            b'L' => &[(0, -1), (1, 0)],
            b'J' => &[(0, -1), (-1, 0)],
            b'F' => &[(0, 1), (1, 0)],
            b'7' => &[(0, 1), (-1, 0)],
            b'S' => &[(0, -1), (0, 1), (-1, 0), (1, 0)],
            _ => &[],
        }
    }

    fn traverse(s: (usize, usize), p: (usize, usize), g: &Vec<&[u8]>, g_idx: &mut Vec<Vec<bool>>) {
        if p.0 < g[0].len() && p.1 < g.len() && !g_idx[p.1][p.0] {
            g_idx[p.1][p.0] = true;
            let b = connects(g[p.1][p.0]);
            if b.contains(&(s.0 as isize - p.0 as isize, s.1 as isize - p.1 as isize)) {
                for i in b.iter() {
                    let a = ((p.0 as isize + i.0) as usize, (p.1 as isize + i.1) as usize);
                    if a != s { traverse(p, a, g, g_idx); }
                }
            }
        }
    }

    traverse(s, (s.0-1, s.1), &g, &mut g_idx);
    traverse(s, (s.0+1, s.1), &g, &mut g_idx);
    traverse(s, (s.0, s.1-1), &g, &mut g_idx);
    traverse(s, (s.0, s.1+1), &g, &mut g_idx);

    // even-odd rule
    let mut inner = 0;
    for (gy, iy) in g.iter().zip(g_idx.iter()) {
        let mut even_odd = false;
        for (gx, ix) in gy.iter().zip(iy.iter()) {
            if matches!(gx, b'|' | b'L' | b'J' | b'F' | b'7' | b'S') && *ix {
                even_odd ^= true;
            } else if even_odd && !ix {
                inner += 1;
                println!("{:?}", *gx as char);
            }
        }
    }

    println!("{g_idx:?}");

    inner.to_string()
}
