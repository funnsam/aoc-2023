pub fn part_1(file: &str) -> String {
    let mut sum = 0;

    'games: for (i, game) in file.lines().enumerate() {
        let game = game.split_once(": ").unwrap().1;

        for i in game.split([',', ';']) {
            let cube = i.trim().split_once(' ').unwrap();
            let n: usize = cube.0.parse().unwrap();
            let l = match cube.1 {
                "red" => 12, "green" => 13,
                "blue" => 14, _ => panic!()
            };

            if n > l {
                continue 'games;
            }
        }

        sum += i+1;
    }

    sum.to_string()
}

pub fn part_2(file: &str) -> String {
    let mut sum = 0;

    for game in file.lines() {
        let game = game.split_once(": ").unwrap().1;

        let mut r = 0;
        let mut g = 0;
        let mut b = 0;

        for i in game.split([',', ';']) {
            let cube = i.trim().split_once(' ').unwrap();
            let n: usize = cube.0.parse().unwrap();
            let color = match cube.1 {
                "red" => &mut r, "green" => &mut g,
                "blue" => &mut b, _ => panic!()
            };

            if *color < n {
                *color = n;
            }
        }

        sum += r*g*b;
    }

    sum.to_string()
}
