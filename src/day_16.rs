#[derive(Debug, Clone)]
struct Beam {
    pos_x: isize,
    pos_y: isize,
    dir_x: isize,
    dir_y: isize,
}

fn beam(g: &Vec<&[u8]>, px: isize, py: isize, dx: isize, dy: isize) -> usize {
    let mut stept = vec![vec![false; g[0].len()]; g.len()];

    let mut hist = Vec::with_capacity(1000);

    let mut beams = vec![Beam {
        pos_x: px,
        pos_y: py,
        dir_x: dx,
        dir_y: dy,
    }];

    while beams.len() != 0 {
        let mut new_beams = beams.clone();
        let mut removed = 0;
        'a: for bi in 0..beams.len() {
            let b = &mut beams[bi];
            stept[b.pos_y as usize][b.pos_x as usize] = true;
            macro_rules! update {
                () => {{
                    b.pos_x += b.dir_x;
                    b.pos_y += b.dir_y;
                    if b.pos_x < 0 || b.pos_y < 0 || b.pos_x as usize >= g[0].len() || b.pos_y as usize >= g.len() {
                        new_beams.remove(bi-removed);
                        removed += 1;
                        continue 'a;
                    }
                }};
            }

            macro_rules! new {
                ($dx: expr, $dy: expr) => {{
                    let px = b.pos_x+$dx;
                    let py = b.pos_y+$dy;
                    if px >= 0 && py >= 0 && (px as usize) < g[0].len() && (py as usize) < g.len() && !hist.contains(&(px as usize, py as usize, $dx, $dy)) {
                        new_beams.push(Beam {
                            pos_x: px,
                            pos_y: py,
                            dir_x: $dx,
                            dir_y: $dy,
                        });
                        hist.push((px as usize, py as usize, $dx, $dy));
                    }
                }};
            }

            macro_rules! update_hist {
                () => {
                    if hist.contains(&(b.pos_x as usize, b.pos_y as usize, b.dir_x, b.dir_y)) {
                        new_beams.remove(bi-removed);
                        removed += 1;
                        continue 'a;
                    }

                    hist.push((b.pos_x as usize, b.pos_y as usize, b.dir_x, b.dir_y));
                };
            }

            match g[b.pos_y as usize][b.pos_x as usize] {
                b'/' => {
                    let x = b.dir_x;
                    let y = b.dir_y;
                    b.dir_x = -y;
                    b.dir_y = -x;
                    update!();
                },
                b'\\' => {
                    let x = b.dir_x;
                    let y = b.dir_y;
                    b.dir_x = y;
                    b.dir_y = x;
                    update!();
                },
                b'-' => {
                    if b.dir_y != 0 {
                        new!(-1, 0);
                        new!(1, 0);
                        update_hist!();
                        new_beams.remove(bi-removed);
                        removed += 1;
                        continue 'a;
                    } else {
                        update!();
                    }
                },
                b'|' => {
                    if b.dir_x != 0 {
                        new!(0, -1);
                        new!(0, 1);
                        update_hist!();
                        new_beams.remove(bi-removed);
                        removed += 1;
                        continue 'a;
                    } else {
                        update!();
                    }
                },
                _ => update!(),
            }

            update_hist!();
            new_beams[bi-removed] = b.clone();
        }

        beams = new_beams;
    }

    stept.iter().fold(0, |a, e|
        e.iter().fold(a, |a, e| a + *e as usize)
    )
}

pub fn part_1(file: &str) -> String {
    let g = file.lines().map(|a| a.as_bytes()).collect::<Vec<&[u8]>>();
    beam(&g, 0, 0, 1, 0).to_string()
}

pub fn part_2(file: &str) -> String {
    let g = file.lines().map(|a| a.as_bytes()).collect::<Vec<&[u8]>>();
    let mut max = 0;
    for i in 0..g[0].len() {
        max = max.max(beam(&g, i as isize, 0, 0, 1));
        max = max.max(beam(&g, i as isize, g.len() as isize-1, 0, -1));
    }
    for i in 0..g.len() {
        max = max.max(beam(&g, 0, i as isize, 1, 0));
        max = max.max(beam(&g, g.len() as isize-1, i as isize, -1, 0));
    }
    max.to_string()
}
