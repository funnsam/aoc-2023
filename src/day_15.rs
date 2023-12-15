fn hash(a: &str) -> u8 {
    let mut t = 0;

    for i in a.as_bytes().iter() {
        t += i;
        t *= 17;
    }

    t
}

pub fn part_1(file: &str) -> String {
    let mut t = 0;

    let f = file.split_once("\n").unwrap_or((file, "")).0;
    for i in f.split(",") {
        t += hash(i) as usize;
    }

    t.to_string()
}

pub fn part_2(file: &str) -> String {
    let mut boxes: Vec<Vec<(String, u8)>> = vec![Vec::with_capacity(10); 256];

    let f = file.split_once("\n").unwrap_or((file, "")).0;
    'a: for i in f.split(",") {
        let l = i.split_once("=").unwrap_or((&i[..i.len()-1], "")).0;
        let h = hash(l);
        match i.as_bytes().last().unwrap() {
            b'-' => {
                for (i, (k, _)) in boxes[h as usize].iter().enumerate() {
                    if k == l {
                        boxes[h as usize].remove(i);
                        break;
                    }
                }
            },
            _ => {
                let n = i.split_once('=').unwrap().1.parse().unwrap();
                for (k, v) in boxes[h as usize].iter_mut() {
                    if k == l {
                        *v = n;
                        continue 'a;
                    }
                }

                boxes[h as usize].push((l.to_string(), n));
            },
        }
    }

    let mut t = 0;
    for (bi, b) in boxes.iter().enumerate() {
        for (li, l) in b.iter().enumerate() {
            t += (bi+1) * (li + 1) * l.1 as usize;
        }
    }

    t.to_string()
}
