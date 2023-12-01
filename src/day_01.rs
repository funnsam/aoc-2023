pub fn task_1(file: String) {
    let mut sum = 0;
    for l in file.lines() {
        let mut this = 0;
        for i in l.chars() {
            if i.is_ascii_digit() {
                this = (i as usize - '0' as usize) * 10;
                break;
            }
        }

        for i in l.chars().rev() {
            if i.is_ascii_digit() {
                this += i as usize - '0' as usize;
                break;
            }
        }

        println!("{this}");

        sum += this
    }

    report!("{sum}");
}

pub fn task_2(file: String) {
    task_1(
        file.replace("one"  , "o1e")
            .replace("two"  , "t2o")
            .replace("three", "t3e")
            .replace("four" , "f4r")
            .replace("five" , "f5e")
            .replace("six"  , "s6x")
            .replace("seven", "s7n")
            .replace("eight", "e8t")
            .replace("nine" , "n9e")
    )
}
