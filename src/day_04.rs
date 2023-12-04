pub fn task_1(file: &str) -> String {
    let mut sum = 0;

    let file = file.as_bytes();

    for l in file.chunks(117).map(|a| &a[10..]) {
        let mut winning: Vec<bool> = vec![false; 100];
        let mut matches = 0;

        for i in 0..10 {
            let i = &l[3 * i..=1 + 3 * i];
            winning[(i[0].saturating_sub(0x30) as usize) * 10 + (i[1] as usize - 0x30)] = true;
        }

        for i in 0..25 {
            let i = &l[32 + 3 * i..=33 + 3 * i];
            matches += winning[(i[0].saturating_sub(0x30) as usize) * 10 + (i[1] as usize - 0x30)] as usize;
        }

        sum += if matches != 0 {
            1 << (matches-1)
        } else { 0 };
    }

    sum.to_string()
}

pub fn task_2(file: &str) -> String {
    let file = file.as_bytes();

    let max_cards = file.len() / 117;

    let mut copies = vec![1; max_cards];

    for (i, l) in file.chunks(117).map(|a| &a[10..]).enumerate() {
        let mut winning = vec![false; 100];
        let mut matches = 0;

        for i in 0..10 {
            let i = &l[3 * i..=1 + 3 * i];
            winning[(i[0].saturating_sub(0x30) as usize) * 10 + (i[1] as usize - 0x30)] = true;
        }

        for i in 0..25 {
            let i = &l[32 + 3 * i..=33 + 3 * i];
            matches += winning[(i[0].saturating_sub(0x30) as usize) * 10 + (i[1] as usize - 0x30)] as usize;
        }

        let n = copies[i];
        for m in 1..=matches {
            copies[m+i] += n;
        }
    }

    let mut sum = 0;

    for i in copies {
        sum += i;
    }

    sum.to_string()
}
