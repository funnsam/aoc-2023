pub fn part_1(file: &str) -> String {
    let mut sum = 0;

    let file = file.as_bytes();
    let mut winning: [std::mem::MaybeUninit<bool>; 0x9F] = [std::mem::MaybeUninit::uninit(); 0x9F];

    for l in file.chunks(117).map(|a| &a[10..]) {
        let mut matches = 0;
        winning.iter_mut().for_each(|a| *a = std::mem::MaybeUninit::new(false));


        for i in 0..10 {
            let i = &l[3 * i..=1 + 3 * i];
            winning[(i[0] << 4) as usize + (i[1] as usize & 0xF)] = std::mem::MaybeUninit::new(true);
        }

        for i in 0..25 {
            let i = &l[32 + 3 * i..=33 + 3 * i];
            matches += unsafe { winning[(i[0] << 4) as usize + (i[1] as usize & 0xF)].assume_init() } as usize;
        }

        sum += if matches != 0 {
            1 << (matches-1)
        } else { 0 };
    }

    sum.to_string()
}

pub fn part_2(file: &str) -> String {
    let file = file.as_bytes();

    let max_cards = file.len() / 117;

    let mut copies = vec![1; max_cards];
    let mut winning: [std::mem::MaybeUninit<bool>; 0x9F] = [std::mem::MaybeUninit::uninit(); 0x9F];

    for (i, l) in file.chunks(117).map(|a| &a[10..]).enumerate() {
        let mut matches = 0;
        winning.iter_mut().for_each(|a| *a = std::mem::MaybeUninit::new(false));

        for i in 0..10 {
            let i = &l[3 * i..=1 + 3 * i];
            winning[(i[0] << 4) as usize + (i[1] as usize & 0xF)] = std::mem::MaybeUninit::new(true);
        }

        for i in 0..25 {
            let i = &l[32 + 3 * i..=33 + 3 * i];
            matches += unsafe { winning[(i[0] << 4) as usize + (i[1] as usize & 0xF)].assume_init() } as usize;
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
