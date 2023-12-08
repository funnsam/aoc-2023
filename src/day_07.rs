const STRENGTHS_1: [usize; 48] = [ // i - 0x30
     0,  0,  1,  2,  3,  4,  5,  6,  7,  8,  9,  0,  0,  0,  0,  0,
     0, 14,  0,  0,  0,  0,  0,  0,  0,  0, 11, 13,  0,  0,  0,  0,
     0, 12,  0,  0, 10,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
];

const STRENGTHS_2: [usize; 48] = [ // i - 0x30
     0,  0,  2,  3,  4,  5,  6,  7,  8,  9, 10,  0,  0,  0,  0,  0,
     0, 14,  0,  0,  0,  0,  0,  0,  0,  0,  1, 13,  0,  0,  0,  0,
     0, 12,  0,  0, 11,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
];

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq)]
enum HandCase {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

pub fn task_1(file: &str) -> String {
    let mut hands = file
        .lines()
        .map(|a| {
            let b = a.as_bytes();
            let i = b[0..5].iter().map(|a| STRENGTHS_1[*a as usize - 0x30]).fold(0, |a, e| (a << 4) | e as u32);
            let j = unsafe { std::str::from_utf8_unchecked(&b[6..]) }.parse().unwrap();
            let mut strengths = [0; 14];
            let mut matcher = i;

            for _ in 0..5 {
                strengths[(matcher & 0xF) as usize - 1] += 1;
                matcher >>= 4;
            }

            let pairs = strengths.iter().filter(|a| **a == 2).count();
            let tris  = strengths.iter().filter(|a| **a == 3).count();
            let quads = strengths.iter().filter(|a| **a == 4).count();
            let quins = strengths.iter().filter(|a| **a == 5).count();

            (
                if quins == 1 { HandCase::FiveOfAKind }
                else if quads == 1 { HandCase::FourOfAKind }
                else if tris  == 1 && pairs == 1 { HandCase::FullHouse }
                else if tris  == 1 { HandCase::ThreeOfAKind }
                else if pairs == 2 { HandCase::TwoPair }
                else if pairs == 1 { HandCase::OnePair }
                else { HandCase::HighCard },

                i, j
            )
        })
        .collect::<Vec<(HandCase, u32, usize)>>();

    use std::cmp::Ordering;
    hands.sort_by(|a, b| {
        if a.0 != b.0 {
            a.0.cmp(&b.0)
        } else {
            for i in 0..5 {
                let mask = 0xF << ((4 - i) << 2);
                if a.1 & mask != b.1 & mask {
                    return (a.1 & mask).cmp(&(b.1 & mask));
                }
            }

            Ordering::Equal
        }
    });

    let mut total = 0;

    for (i, (_, _, bid)) in hands.iter().enumerate() {
        total += bid * (i + 1);
    }

    total.to_string()
}

pub fn task_2(file: &str) -> String {
    let mut hands = file
        .lines()
        .map(|a| {
            let b = a.as_bytes();
            let i = b[0..5].iter().map(|a| STRENGTHS_2[*a as usize - 0x30]).fold(0, |a, e| (a << 4) | e as u32);
            let k = unsafe { std::str::from_utf8_unchecked(&b[6..]) }.parse().unwrap();

            let mut strengths = [0; 14];
            let mut matcher = i;

            for _ in 0..5 {
                strengths[(matcher & 0xF) as usize - 1] += 1;
                matcher >>= 4;
            }

            let j = strengths[0];

            let pairs = strengths.iter().filter(|a| **a == 2).count();
            let quins = strengths.iter().filter(|a| **a == 5).count();

            let sngls_j = strengths[1..].iter().filter(|a| **a == 1).count();
            let pairs_j = strengths[1..].iter().filter(|a| **a == 2).count();
            let tris_j  = strengths[1..].iter().filter(|a| **a == 3).count();
            let quads_j = strengths[1..].iter().filter(|a| **a == 4).count();

            (
                if      quins == 1 || (quads_j >= 1 && j >= 1) || (tris_j >= 1 && j >= 2) || (pairs_j >= 1 && j >= 3) || (sngls_j >= 1 && j >= 4) { HandCase::FiveOfAKind }
                else if               (quads_j >= 1 && j >= 0) || (tris_j >= 1 && j >= 1) || (pairs_j >= 1 && j >= 2) || (sngls_j >= 1 && j >= 3) { HandCase::FourOfAKind }
                else if (tris_j == 1 && pairs_j == 1 && j == 0) || (pairs_j == 2 && j == 1) { HandCase::FullHouse }
                else if                                           (tris_j >= 1 && j >= 0) || (pairs_j >= 1 && j >= 1) || (sngls_j >= 1 && j >= 2) { HandCase::ThreeOfAKind }
                else if pairs == 2                             { HandCase::TwoPair }
                else if pairs == 1 || (sngls_j >= 1 && j == 1) { HandCase::OnePair }
                else { HandCase::HighCard },
                i, k
            )
        })
        .collect::<Vec<(HandCase, u32, usize)>>();

    use std::cmp::Ordering;
    hands.sort_by(|a, b| {
        if a.0 != b.0 {
            a.0.cmp(&b.0)
        } else {
            for i in 0..5 {
                let mask = 0xF << ((4 - i) << 2);
                if a.1 & mask != b.1 & mask {
                    return (a.1 & mask).cmp(&(b.1 & mask));
                }
            }

            Ordering::Equal
        }
    });

    let mut total = 0;

    for (i, (_, _, bid)) in hands.iter().enumerate() {
        total += bid * (i + 1);
    }

    total.to_string()
}
