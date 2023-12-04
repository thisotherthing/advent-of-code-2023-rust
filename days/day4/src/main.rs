fn main() {
    assert_eq!(part_a(include_str!("example.txt")), 13);
    assert_eq!(part_a(include_str!("input.txt")), 21821);
    assert_eq!(part_b(include_str!("example.txt")), 30);
    assert_eq!(part_b(include_str!("input.txt")), 5539496);
}

fn part_a(input: &str) -> i64 {
    input
        .trim()
        .lines()
        .map(|line| {
            let (_card, values) = line.split_once(": ").expect("split line");

            let (winning_numbers_string, numbers_string) =
                values.split_once(" | ").expect("ssplit values");

            // println!("[{}] [{}]", winning_numbers_string, numbers_string);

            let winning_numbers: Vec<i64> = winning_numbers_string
                .split_whitespace()
                .map(|number| number.parse::<i64>().expect("parse winning number"))
                .collect();
            numbers_string
                .split_ascii_whitespace()
                .map(|number| number.parse::<i64>().expect("parse number"))
                .filter(|number| winning_numbers.contains(number))
                .count()
        })
        .map(|line| -> i64 {
            // dbg!(line);
            let val: u32 = line.try_into().unwrap();
            2i64.pow(val - 1u32)

            // if line > 0 {
            //     dbg!(pow);
            //     return pow;
            // }
            // 0
        })
        .sum()
}

fn part_b(input: &str) -> usize {
    let mut card_counts: Vec<usize> = vec![1; input.trim().lines().count()];
    let mut wins: Vec<usize> = vec![0; input.trim().lines().count()];

    input.trim().lines().enumerate().for_each(|(index, line)| {
        let (_card, values) = line.split_once(": ").expect("split line");

        let (winning_numbers_string, numbers_string) =
            values.split_once(" | ").expect("split values");

        // println!("[{}] [{}]", winning_numbers_string, numbers_string);

        let winning_numbers: Vec<i64> = winning_numbers_string
            .split_whitespace()
            .map(|number| number.parse::<i64>().expect("parse winning number"))
            .collect();
        wins[index] += numbers_string
            .split_ascii_whitespace()
            .map(|number| number.parse::<i64>().expect("parse number"))
            .filter(|number| winning_numbers.contains(number))
            .count()
    });

    let mut total_cards = 0;

    for i in 0..card_counts.len() {
        for _ in 0..card_counts[i] {
            for other in 1..=wins[i] {
                if i + other < card_counts.len() {
                    card_counts[i + other] += 1;
                }
            }
        }

        total_cards += card_counts[i];
    }

    // dbg!(wins);
    // dbg!(card_counts);

    total_cards
}
