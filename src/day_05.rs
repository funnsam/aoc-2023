pub fn task_1(file: &str) -> String {
    let mut file = file.lines();
    let seeds = file.next().unwrap().split_whitespace().skip(1).map(|a| a.parse().unwrap()).collect::<Vec<usize>>();

    file.next();
    file.next();

    let mut map_at = 0;
    let mut maps = vec![vec![]; 7];

    while let Some(l) = file.next() {
        if l.is_empty() {
            map_at += 1;
            file.next();
        } else {
            let mut s = l.split_whitespace();
            let a = |a: &mut std::str::SplitWhitespace| a.next().unwrap().parse::<usize>().unwrap();
            maps[map_at].push((a(&mut s), a(&mut s), a(&mut s)));
        }
    }

    let mut min = usize::MAX;

    for s in seeds.into_iter() {
        let mut current = s;
        'maps: for i in maps.iter() {
            for (j, k, l) in i.iter() {
                if (*k..k+l).contains(&current) {
                    current = j + (current - k);
                    continue 'maps;
                }
            }
        }

        if min > current { min = current; }
    }

    min.to_string()
}

pub fn task_2(file: &str) -> String {
    let mut file = file.lines();
    let mut seeds = file.next().unwrap()
        .split_whitespace()
        .skip(1)
        .map(|a| a.parse().unwrap())
        .array_chunks()
        .map(|a: [usize; 2]| [a[0], a[0]+a[1]-1])
        .collect::<Vec<[usize; 2]>>();

    file.next();
    file.next();

    let mut map_at = 0;
    let mut maps = vec![vec![]; 7];

    while let Some(l) = file.next() {
        if l.is_empty() {
            map_at += 1;
            file.next();
        } else {
            let mut s = l.split_whitespace();
            let a = |a: &mut std::str::SplitWhitespace| a.next().unwrap().parse::<usize>().unwrap();
            maps[map_at].push((a(&mut s), a(&mut s), a(&mut s)));
        }
    }

    let mut min = usize::MAX;

    seeds.sort_by(|a, b| {
        if a[0] < b[0] {
            a[1].cmp(&b[1])
        } else {
            a[0].cmp(&b[0])
        }
    });

    let mut new_seeds = Vec::new();
    let mut now = seeds[0];
    for range in seeds.iter().skip(1) {
        if now[1] + 1 < range[0] {
            new_seeds.push([now[0], now[1]]);
            now = range.clone();
        } else {
            now[1] = now[1].max(range[1]);
        }
    }
    new_seeds.push([now[0], now[1]]);

    for s in new_seeds.into_iter() {
        for s2 in s[0]..s[1] {
            let mut current = s2;
            'maps: for i in maps.iter() {
                for (j, k, l) in i.iter() {
                    if (*k..k+l).contains(&current) {
                        current = j + (current - k);
                        continue 'maps;
                    }
                }
            }

            if min > current { min = current; }
        }
    }

    min.to_string()
}
