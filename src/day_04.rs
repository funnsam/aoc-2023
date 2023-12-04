pub fn task_1(file: String) {
    let mut sum = 0;

    for l in file.lines() {
        let l = l.split_once(": ").unwrap().1;
        let mut m = false;

        let mut winning: Vec<usize> = Vec::new();
        let mut matches = 0;
        for i in l.split_whitespace() {
            if !m {
                if i == "|" {
                    m = true;
                } else {
                    winning.push(i.parse().unwrap());
                }
            } else {
                if winning.contains(&i.parse().unwrap()) {
                    matches += 1;
                }
            }
        }

        sum += if matches != 0 {
            2_usize.pow(matches-1)
        } else { 0 };
    }

    report!("{sum}");
}

pub fn task_2(file: String) {
    use std::collections::HashMap;

    let mut copies = HashMap::new();
    let mut max = 0;

    for (i, l) in file.lines().enumerate() {
        max = i+1;

        let l = l.split_once(": ").unwrap().1;
        let mut m = false;

        let mut winning: Vec<usize> = Vec::new();
        let mut matches = 0;
        for i in l.split_whitespace() {
            if !m {
                if i == "|" {
                    m = true;
                } else {
                    winning.push(i.parse().unwrap());
                }
            } else {
                if winning.contains(&i.parse().unwrap()) {
                    matches += 1;
                }
            }
        }

        let n = copies.get(&i).unwrap_or(&0)+1;
        for m in 1..=matches {
            copies.entry(i+m).and_modify(|a| *a += n).or_insert(n);
        }
    }

    let mut sum = 0;

    for i in 1..=max {
        sum += copies.get(&i).unwrap_or(&0) + 1;
    }

    report!("{sum}");
}
