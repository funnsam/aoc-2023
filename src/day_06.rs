pub fn task_1(file: &str) -> String {
    let mut product = 1;

    let (t, d) = file.split_once('\n').unwrap();

    for (t, d) in t.split_whitespace().zip(d.split_whitespace()).skip(1) {
        let mut sum = 0;
        let t = t.parse::<usize>().unwrap();
        let d = d.parse::<usize>().unwrap();

        for h in 1..t {
            let b = h * (t-h) > d;
            if !b && (t>>1 < h) { break; }
            sum += b as usize;
        }

        product *= sum.max(1);
    }

    product.to_string()
}

pub fn task_2(file: &str) -> String {
    let file = file.replace(' ', "");
    let (t, d) = file.split_once('\n').unwrap();
    let t = t.split_once(':').unwrap().1;
    let d = d.split_once(':').unwrap().1;

    let t = t.parse::<usize>().unwrap();
    let d = d.trim_end().parse::<usize>().unwrap();

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
    let d = (b*b - 4.0*a*c).sqrt();
    let s = (-b + d) / 2.0*a;
    let e = (-b - d) / 2.0*a;

    (e.floor() as usize - s.floor() as usize).to_string()
}
