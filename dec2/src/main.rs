use input_reader;

fn part1() {
    let content = input_reader::read_file("input");

    let mut score = 0;

    let games = content.split("\r\n");
    for game in games {
        if game.is_empty() {
            continue;
        }
        let plays: Vec<char> = game.chars().collect();
        let opponent = plays[0];
        let me = plays[2];
        let both = (opponent, me);
        // A = X = rock 1
        // B = Y = paper 2
        // C = Z = sissor 3

        // win = 6
        // draw = 3
        // loss = 0

        // score = outcome + option played
        match both {
            ('A', 'X') => score += 3 + 1,
            ('B', 'X') => score += 0 + 1,
            ('C', 'X') => score += 6 + 1,
            ('A', 'Y') => score += 6 + 2,
            ('B', 'Y') => score += 3 + 2,
            ('C', 'Y') => score += 0 + 2,
            ('A', 'Z') => score += 0 + 3,
            ('B', 'Z') => score += 6 + 3,
            ('C', 'Z') => score += 3 + 3,
            _ => panic!("Help"),
        }
    }

    println!("{}", score);
}

fn part2() {
    let content = input_reader::read_file("input");

    let mut score = 0;

    let games = content.split("\r\n");
    for game in games {
        if game.is_empty() {
            continue;
        }
        let plays: Vec<char> = game.chars().collect();
        let opponent = plays[0];
        let me = plays[2];
        let both = (opponent, me);
        // A = rock 1
        // B = paper 2
        // C = sissor 3

        // X = need to lose
        // Y = need to draw
        // Z = need to win

        // win = 6
        // draw = 3
        // loss = 0

        // score = outcome + option played
        match both {
            ('A', 'X') => score += 0 + 3,
            ('A', 'Y') => score += 3 + 1,
            ('A', 'Z') => score += 6 + 2,
            ('B', 'X') => score += 0 + 1,
            ('B', 'Y') => score += 3 + 2,
            ('B', 'Z') => score += 6 + 3,
            ('C', 'X') => score += 0 + 2,
            ('C', 'Y') => score += 3 + 3,
            ('C', 'Z') => score += 6 + 1,
            _ => panic!("Help"),
        }
    }

    println!("{}", score);
}

fn main() {
    part1();
    part2();
}
