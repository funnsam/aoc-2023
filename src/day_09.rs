pub fn task_1(file: &str) -> String {
    let mut sum = 0;

    let mut nums = Vec::with_capacity(100);
    for l in file.lines() {
        nums.clear();
        nums.push(l.split_whitespace().map(|a| a.parse::<isize>().unwrap()).collect::<Vec<isize>>());
        let mut zero = false;
        while !zero {
            zero = true;
            let prev = nums.last().unwrap().clone();
            let mut d = prev[0];
            nums.push(Vec::with_capacity(prev.len()-1));
            for i in prev.iter().skip(1) {
                nums.last_mut().unwrap().push(i - d);
                zero &= (i - d) == 0;
                d = *i;
            }
        }

        nums.reverse();
        for i in nums.iter().skip(1) {
            sum += i.last().unwrap();
        }
    }

    sum.to_string()
}

pub fn task_2(file: &str) -> String {
    let mut sum = 0;

    let mut nums = Vec::with_capacity(100);
    for l in file.lines() {
        nums.clear();
        nums.push(l.split_whitespace().map(|a| a.parse::<isize>().unwrap()).collect::<Vec<isize>>());
        let mut zero = false;
        while !zero {
            zero = true;
            let prev = nums.last().unwrap().clone();
            let mut d = prev[0];
            nums.push(Vec::with_capacity(prev.len()-1));
            for i in prev.iter().skip(1) {
                nums.last_mut().unwrap().push(i - d);
                zero &= (i - d) == 0;
                d = *i;
            }
        }

        nums.reverse();
        let mut c = 0;
        for i in nums.iter().skip(1) {
            c = i[0] - c;
        }

        sum += c;
    }

    sum.to_string()
}
