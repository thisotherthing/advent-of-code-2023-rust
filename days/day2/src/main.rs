use std::collections::HashMap;

fn main() {
    assert_eq!(part_a(include_str!("example.txt")), 8);
    assert_eq!(part_a(include_str!("input.txt")), 2545);
    assert_eq!(part_b(include_str!("example.txt")), 2286);
    assert_eq!(part_b(include_str!("input.txt")), 78111);
}

fn part_a(input: &str) -> i32 {
    let max_dices = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    let mut amount: i32 = 0;

    'gamesLoop: for line in input.trim().lines() {
        let mut split = line.split(": ");

        let game_id = split
            .next()
            .unwrap()
            .split(' ')
            .nth(1)
            .unwrap()
            .parse::<i32>()
            .unwrap();

        for picks in split.next().unwrap().split("; ") {
            for dices in picks.split(", ") {
                let mut dice = dices.split(' ');
                let count = dice.next().unwrap().parse::<i32>().unwrap();
                let color = dice.next().unwrap();

                if count > max_dices[color] {
                    // println!(
                    //     "game '{}' not possible '{}' '{}' max:'{}'",
                    //     game_id, color, count, max_dices[color]
                    // );
                    continue 'gamesLoop;
                }
            }
        }
        amount += game_id
    }

    amount
}

fn part_b(input: &str) -> i32 {
    let mut amount: i32 = 0;

    for line in input.trim().lines() {
        let mut split = line.split(": ");

        let mut min_dices = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);

        for picks in split.nth(1).unwrap().split("; ") {
            for dices in picks.split(", ") {
                let mut dice = dices.split(' ');
                let count = dice.next().unwrap().parse::<i32>().unwrap();
                let color = dice.next().unwrap();

                *min_dices.entry(color).or_insert(0) = min_dices[color].max(count);
            }
        }

        let power: i32 = min_dices.into_values().product();
        amount += power;
    }

    amount
}
