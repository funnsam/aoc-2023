#[derive(Debug, Clone)]
enum Chunk {
    Known(usize), Unknown, Seperater
}

pub fn part_1(file: &str) -> String {
    let spr = file.lines().map(|a| {
        let (b, c) = a.split_once(' ').unwrap();
        (b.trim_end_matches('.').as_bytes(), c.split(',').map(|d| d.parse().unwrap()).collect::<Vec<usize>>())
    });

    let mut sum = 0;
    for s in spr {
        let mut chunks = vec![];
        let mut a = s.0.to_vec();
        a.dedup_by(|a, b| *a == b'.' && *b == b'.');
        for i in a.iter() {
            if *i == b'#' {
                let l = chunks.last_mut();
                match l {
                    Some(Chunk::Known(i)) => *i += 1,
                    _ => chunks.push(Chunk::Known(1)),
                }
            } else if *i == b'?' {
                chunks.push(Chunk::Unknown);
            } else if !matches!(chunks.last(), Some(Chunk::Seperater) | None) {
                chunks.push(Chunk::Seperater);
            }
        }

        let mut poss = vec![chunks];
        let mut done = false;
        while !done {
            done = true;
            let mut np = Vec::with_capacity(poss.len());

            'i: for (idx, i) in poss.iter().enumerate() {
                macro_rules! unknowns {
                    () => {
                        i.iter().filter(|a| matches!(a, Chunk::Unknown)).count()
                    };
                }

                let mut i = i.clone();
                i.dedup_by(|a, b| matches!(a, Chunk::Seperater) && matches!(b, Chunk::Seperater));
                for j in (0..i.len()).rev() {
                    if matches!(i[j], Chunk::Seperater) {
                        i.remove(j);
                    } else {
                        break;
                    }
                }

                let mut counts = s.1.clone();
                'j: for (jdx, j) in i.iter().enumerate().rev() { // solving it reversly
                    match j {
                        Chunk::Known(i) => match counts.last_mut() {
                            Some(j) => if *j >= *i {
                                *j -= i;
                            } else if unknowns!() == 0 { continue 'i; },
                            None => if unknowns!() == 0 { continue 'i; },
                        },
                        Chunk::Unknown => {
                            // let j = match counts.last_mut() {
                            //     Some(j) => j,
                            //     None => continue 'i,
                            // };
                            // collapse into branches
                            let mut a = i.clone();
                            let mut b = i.clone();
                            a[jdx] = Chunk::Known(1);
                            b[jdx] = Chunk::Seperater;
                            np.push(a);
                            np.push(b);
                            done = false;
                            continue 'i;
                        },
                        Chunk::Seperater => {
                            match counts.last() {
                                Some(0) => { counts.pop(); },
                                _ => if unknowns!() == 0 { continue 'i; },
                            }
                        },
                    }
                }

                match counts.last() {
                    Some(0) => { counts.pop(); },
                    _ => {},
                }

                if counts.is_empty() || unknowns!() != 0 {
                    np.push(i.clone());
                }

                if unknowns!() != 0 {
                    done = false;
                }
            }

            poss = np;
        }

        sum += poss.len();
    }
    sum.to_string()
}

pub fn part_2(file: &str) -> String {
    todo!();
}
