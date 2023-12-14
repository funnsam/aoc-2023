fn get_load(g: &Vec<Vec<u8>>) -> usize {
    let mut load = 0;

    for (yi, y) in g.iter().enumerate() {
        for x in y.iter() {
            if *x == b'O' {
                load += g.len() - yi;
            }
        }
    }

    load
}

pub fn part_1(file: &str) -> String {
    let mut g = file.lines().map(|a| a.as_bytes().to_vec()).collect::<Vec<Vec<u8>>>();

    for yi in 0..g.len() {
        'a: for xi in 0..g[yi].len() {
            if yi != 0 && g[yi][xi] == b'O' {
                // roll it
                for yj in (1..=yi).rev() {
                    if g[yj-1][xi] != b'.' {
                        // stop the stone
                        g[yi][xi] = b'.';
                        g[yj][xi] = b'O';
                        continue 'a;
                    }
                }

                // didn't encounter another stone
                g[yi][xi] = b'.';
                g[0][xi] = b'O';
            }
        }
    }

    get_load(&g).to_string()
}

pub fn part_2(file: &str) -> String {
    use std::collections::HashMap;

    const CALCULATE_CYCLE: usize = 1000;

    let mut g = file.lines().map(|a| a.as_bytes().to_vec()).collect::<Vec<Vec<u8>>>();

    let mut buf = HashMap::with_capacity(CALCULATE_CYCLE);
    let mut hist = Vec::with_capacity(CALCULATE_CYCLE);

    for i in 0..CALCULATE_CYCLE {
        // ^
        for yi in 0..g.len() {
            'a: for xi in 0..g[yi].len() {
                if yi != 0 && g[yi][xi] == b'O' {
                    // roll it
                    for yj in (1..=yi).rev() {
                        if g[yj-1][xi] != b'.' {
                            // stop the stone
                            g[yi][xi] = b'.';
                            g[yj][xi] = b'O';
                            continue 'a;
                        }
                    }

                    // didn't encounter anotheri, ok := m["route"] stone
                    g[yi][xi] = b'.';
                    g[0][xi] = b'O';
                }
            }
        }

        // <
        for yi in 0..g.len() {
            'a: for xi in 0..g[yi].len() {
                if xi != 0 && g[yi][xi] == b'O' {
                    // roll it
                    for xj in (1..=xi).rev() {
                        if g[yi][xj-1] != b'.' {
                            // stop the stone
                            g[yi][xi] = b'.';
                            g[yi][xj] = b'O';
                            continue 'a;
                        }
                    }

                    // didn't encounter another stone
                    g[yi][xi] = b'.';
                    g[yi][0] = b'O';
                }
            }
        }

        // v
        for yi in (0..g.len()).rev() {
            'a: for xi in (0..g[yi].len()).rev() {
                if yi != g.len()-1 && g[yi][xi] == b'O' {
                    // roll it
                    for yj in yi+1..g.len() {
                        if g[yj][xi] != b'.' {
                            // stop the stone
                            g[yi][xi] = b'.';
                            g[yj-1][xi] = b'O';
                            continue 'a;
                        }
                    }

                    // didn't encounter another stone
                    g[yi][xi] = b'.';
                    let glen = g.len();
                    g[glen-1][xi] = b'O';
                }
            }
        }

        // >
        for yi in (0..g.len()).rev() {
            'a: for xi in (0..g[yi].len()).rev() {
                if xi != g[yi].len()-1 && g[yi][xi] == b'O' {
                    // roll it
                    for xj in xi+1..g[yi].len() {
                        if g[yi][xj] != b'.' {
                            // stop the stone
                            g[yi][xi] = b'.';
                            g[yi][xj-1] = b'O';
                            continue 'a;
                        }
                    }

                    // didn't encounter another stone
                    g[yi][xi] = b'.';
                    let gyilen = g[yi].len();
                    g[yi][gyilen-1] = b'O';
                }
            }
        }

        if buf.contains_key(&g) {
            let j = *buf.get(&g).unwrap();
            return (hist[(1000000000_usize-j) % (i-j) + (j-1_usize)] as usize).to_string(); // stupid rust error
        } else {
            hist.push(get_load(&g));
            buf.insert(g.clone(), i);
        }
    }

    get_load(&g).to_string()
}
