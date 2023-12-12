#[derive(Debug)]
enum Chunk {
    Known(usize), Unknown(usize), Seperater
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
                let l = chunks.last_mut();
                match l {
                    Some(Chunk::Unknown(i)) => *i += 1,
                    _ => chunks.push(Chunk::Unknown(1)),
                }
            } else if !matches!(chunks.last(), Some(Chunk::Seperater) | None) {
                chunks.push(Chunk::Seperater);
            }
        }
        println!("{chunks:?}");
    }
    sum.to_string()
}

pub fn part_2(file: &str) -> String {
    todo!();
}
