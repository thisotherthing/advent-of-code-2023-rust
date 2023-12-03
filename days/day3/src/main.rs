use std::collections::HashSet;

fn main() {
    assert_eq!(part_a(include_str!("example.txt")), 4361);
    assert_eq!(part_a(include_str!("input.txt")), 530849);
    // assert_eq!(part_b(include_str!("example.txt")), 467835);
    // assert_eq!(part_b(include_str!("input.txt")), 0);
}

fn get_index(x: usize, y: usize, width: usize) -> usize {
    x + y * width
}

fn is_touching_symbol(
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    schematic: &Vec<char>,
    symbols: &HashSet<char>,
) -> bool {
    let mut touches: bool = false;
    for x_offset in -1..=1 {
        for y_offset in -1..=1 {
            if x_offset == 0 && y_offset == 0 {
                continue;
            }

            let x = x.checked_add_signed(x_offset);
            let y = y.checked_add_signed(y_offset);

            if let Some(x) = x {
                if let Some(y) = y {
                    let x = x.min(width - 1);
                    let y = y.min(height - 1);

                    // println!(
                    //     "{} {} {}",
                    //     x_offset,
                    //     y_offset,
                    //     schematic[get_index(x, y, width)]
                    // );

                    touches = touches || symbols.contains(&schematic[get_index(x, y, width)]);
                }
            }
        }
    }

    touches
}

fn part_a(input: &str) -> i64 {
    let mut amount: i64 = 0;

    let width = input.split_once('\n').unwrap().0.len();
    let height = input.trim().lines().count();
    let mut schematic = Vec::with_capacity(width * height);
    let mut symbols = HashSet::new();

    for line in input.trim().lines() {
        for c in line.trim().chars() {
            schematic.push(c);

            if c != '.' && !c.is_ascii_digit() && !symbols.contains(&c) {
                symbols.insert(c);
            }
        }
    }

    // dbg!(&symbols);

    let mut number_builder: Vec<char> = Vec::new();
    let mut touches_symbol: bool;
    for y in 0..height {
        number_builder.clear();
        touches_symbol = false;
        for x in 0..width {
            let char = schematic[get_index(x, y, width)];

            // if it's a number check if it's touching a symbol, add to builder, and keep going
            if char.is_numeric() {
                touches_symbol =
                    touches_symbol || is_touching_symbol(x, y, width, height, &schematic, &symbols);
                number_builder.push(char);

                continue;
            }

            // track number
            if !number_builder.is_empty() {
                let number = number_builder
                    .clone()
                    .into_iter()
                    .collect::<String>()
                    .parse::<i64>()
                    .unwrap();

                // println!("{} {}", &number, &touches_symbol);

                if touches_symbol {
                    amount += number;
                }

                number_builder.clear();
                touches_symbol = false;
            }
        }

        if !number_builder.is_empty() {
            let number = number_builder
                .clone()
                .into_iter()
                .collect::<String>()
                .parse::<i64>()
                .unwrap();

            // println!("{} {}", &number, &touches_symbol);

            if touches_symbol {
                amount += number;
            }
        }
    }

    amount
}

// fn part_b(input: &str) -> i64 {
//     0
// }
