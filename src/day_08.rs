pub fn part_1(file: &str) -> String {
    use std::collections::HashMap;

    let (steps, route_raw) = file.split_once("\n\n").unwrap();
    let steps = steps.as_bytes().iter().map(|a| *a == b'R').collect::<Vec<bool>>();

    let mut route: HashMap<u32, [u32; 2]> = HashMap::with_capacity(1024);
    for l in route_raw.as_bytes().chunks(17) {
        route.insert(
            u32::from_le_bytes([l[0], l[1], l[2], 0]), [
            u32::from_le_bytes([l[7], l[8], l[9], 0]),
            u32::from_le_bytes([l[12], l[13], l[14], 0]),
        ]);
    }

    let mut count = 0;
    let mut current = u32::from_be_bytes(*b"\0AAA");
    let sl = steps.len();
    loop {
        let s = steps[count % sl];
        let a = route[&current][s as usize];

        count += 1;
        current = a;

        if a == u32::from_be_bytes(*b"\0ZZZ") {
            return count.to_string()
        }
    }
}

pub fn part_2(file: &str) -> String {
    use std::collections::HashMap;

    let (steps, route_raw) = file.split_once("\n\n").unwrap();
    let steps = steps.as_bytes().iter().map(|a| *a == b'R').collect::<Vec<bool>>();

    let mut route: HashMap<u32, [u32; 2]> = HashMap::with_capacity(1024);
    for l in route_raw.as_bytes().chunks(17) {
        route.insert(
            u32::from_le_bytes([l[0], l[1], l[2], 0]), [
            u32::from_le_bytes([l[7], l[8], l[9], 0]),
            u32::from_le_bytes([l[12], l[13], l[14], 0]),
        ]);
    }

    let mut counts = Vec::with_capacity(6);
    let sl = steps.len();

    // good time to parallelise
    for g in route
        .iter()
        .filter(|(a, _)| (**a >> 16) as u8 == b'A')
        .map(|(a, _)| a.clone()) {

        let mut count = 0;
        let mut current = g;
        loop {
            let s = steps[count % sl];
            let a = route[&current][s as usize];

            current = a;

            count += 1;
            if (a >> 16) as u8 == b'Z' {
                counts.push(count);
                break;
            }
        }
    }

    lcm(&counts).to_string()
}

fn lcm(c: &[usize]) -> usize {
    if c.len() == 1 {
        return c[0];
    }

    let a = c[0];
    let b = lcm(&c[1..]);
    a * b / gcf(a, b)
}

fn gcf(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcf(b, a % b)
    }
}
