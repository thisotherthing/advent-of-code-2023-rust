fn main() {
    assert_eq!(part_a(include_str!("example.txt")), 142);
    assert_eq!(part_a(include_str!("input.txt")), 54388);
    assert_eq!(part_b(include_str!("example_b.txt")), 281);
    assert_eq!(part_b(include_str!("input.txt")), 53515);
}

fn part_a(input: &str) -> i64 {
    let mut amount: i64 = 0;

    for line in input.trim().split('\n') {
        // let mut result = re.replace_all(line, "").parse::<i64>().unwrap();
        let result = line.replace(char::is_alphabetic, "");
        let mut chars = result.chars();

        let mut string = String::new();
        string.push(chars.next().unwrap());

        let last_digit = chars.next_back();

        if let Some(last_digit) = last_digit {
            string.push(last_digit);
        }

        let mut number = string.parse::<i64>().unwrap();

        if number < 10 {
            number += number * 10;
        }

        amount += number;
        // string.push(result.nth_back(0).unwrap());
        // dbg!(string.parse::<i64>());
        // dbg!(&number);
    }

    amount
}

fn part_b(input: &str) -> i64 {
    let mut amount: i64 = 0;

    for line in input.trim().split('\n') {
        let result = line
            .replace("one", "one1one")
            .replace("two", "two2two")
            .replace("three", "three3three")
            .replace("four", "four4four")
            .replace("five", "five5five")
            .replace("six", "six6six")
            .replace("seven", "seven7seven")
            .replace("eight", "eight8eight")
            .replace("nine", "nine9nine")
            .replace(char::is_alphabetic, "");
        let mut chars = result.chars();

        let mut string = String::new();
        string.push(chars.next().unwrap());

        let last_digit = chars.next_back();

        if let Some(last_digit) = last_digit {
            string.push(last_digit);
        }

        let mut number = string.parse::<i64>().unwrap();

        if number < 10 {
            number += number * 10;
        }

        amount += number;
        // string.push(result.nth_back(0).unwrap());
        // dbg!(string.parse::<i64>());
        // dbg!(&number);
    }

    amount
}
