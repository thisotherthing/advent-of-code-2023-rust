fn main() {
    assert_eq!(part_a(include_str!("example.txt")), 13);
    assert_eq!(part_a(include_str!("input.txt")), 0);
    // assert_eq!(part_b(include_str!("example.txt")), 0);
    // assert_eq!(part_b(include_str!("input.txt")), 0);
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

// fn part_b(input: &str) -> i64 {
//     0
// }
