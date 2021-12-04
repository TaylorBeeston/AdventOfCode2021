mod input;

use crate::input::get_input;

fn main() {
    let input = get_input();

    let oxygen_generator_rating = oxygen_generator_rating(&input);
    let co2_scrubber_rating = co2_scrubber_rating(&input);

    let og = usize::from_str_radix(&oxygen_generator_rating, 2).unwrap();
    let co2 = usize::from_str_radix(&co2_scrubber_rating, 2).unwrap();

    println!(
        "og {} ({}), co2 {} ({})",
        oxygen_generator_rating, og, co2_scrubber_rating, co2
    );
    println!("Answer {}", og * co2);
}

fn most_common_bits(input: &Vec<&str>) -> String {
    input
        .iter()
        .fold(vec![0; input[0].len()], |mut acc, current| {
            current.chars().enumerate().for_each(|(index, bit)| {
                acc[index] += match bit {
                    '1' => 1,
                    _ => 0,
                }
            });

            acc
        })
        .iter()
        .map(|total| {
            if total >= &((input.len() + 1) / 2) {
                "1"
            } else {
                "0"
            }
        })
        .collect::<Vec<&str>>()
        .join("")
}

fn least_common_bits(input: &Vec<&str>) -> String {
    input
        .iter()
        .fold(vec![0; input[0].len()], |mut acc, current| {
            current.chars().enumerate().for_each(|(index, bit)| {
                acc[index] += match bit {
                    '1' => 1,
                    _ => 0,
                }
            });

            acc
        })
        .iter()
        .map(|total| {
            if total < &((input.len() + 1) / 2) {
                "1"
            } else {
                "0"
            }
        })
        .collect::<Vec<&str>>()
        .join("")
}

fn oxygen_generator_rating<'a>(input: &Vec<&'a str>) -> &'a str {
    let mut values = input.clone();
    let mut index = 0;

    while values.len() > 1 {
        let mcb = most_common_bits(&values);
        values = values
            .iter()
            .filter(|value| value.chars().nth(index) == mcb.chars().nth(index))
            .map(|value| *value)
            .collect();
        index += 1;
    }

    values[0]
}

fn co2_scrubber_rating<'a>(input: &Vec<&'a str>) -> &'a str {
    let mut values = input.clone();
    let mut index = 0;

    while values.len() > 1 {
        let lcb = least_common_bits(&values);
        values = values
            .iter()
            .filter(|value| value.chars().nth(index) == lcb.chars().nth(index))
            .map(|value| *value)
            .collect();
        index += 1;
    }

    values[0]
}
