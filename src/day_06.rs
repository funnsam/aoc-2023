fn parse_ignore_until_at(i: &str, u: u8, k: usize) -> (usize, usize) {
    let mut r = 0;
    let mut e = 0;

    for (j, b) in i.as_bytes().iter().enumerate().skip(k) {
        if *b == u { e = j; break; }
        if b.is_ascii_digit() {
            r = r * 10 + (*b as usize - 0x30)
        }
    }

    (r, e+1)
}

pub fn part_1(file: &str) -> String {
    let mut product = 1;

    let (t, d) = file.split_once('\n').unwrap();

    for (t, d) in t.split_whitespace().zip(d.split_whitespace()).skip(1) {
        let mut sum = 0;
        let (t, _) = parse_ignore_until_at(t, b'\0', 0);
        let (d, _) = parse_ignore_until_at(d, b'\0', 0);

        for h in 1..t {
            let b = h * (t-h) > d;
            if !b && (t>>1 < h) { break; }
            sum += b as usize;
        }

        product *= sum.max(1);
    }

    product.to_string()
}

pub fn part_2(file: &str) -> String {
    let (t, n) = parse_ignore_until_at(file, b'\n', 0);
    let (d, _) = parse_ignore_until_at(file, b'\n', n);

    // x * (t-x) = d
    // xt - x² - d = 0
    // -x² + tx - d = 0
    //
    // Quad Eq: ax² + bx + c = 0
    //
    // a = -1
    // b = t
    // c = -d
    //
    // x will be the starting / ending point

    let a = -1.0;
    let b = t as f64;
    let c = -(d as f64);
    let d = b.mul_add(b, -(4.0*a * c)).sqrt();
    let s = (-b + d) / (2.0*a);
    let e = (-b - d) / (2.0*a);

    (e.floor() as usize - s.floor() as usize).to_string()
}
