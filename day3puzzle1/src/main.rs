mod input;

use crate::input::get_input;

fn main() {
    let input = get_input();

    let mcb = most_common_bits(input);
    let lcb = flip_bits(&mcb);

    let mcb_num = usize::from_str_radix(&mcb, 2).unwrap();
    let lcb_num = usize::from_str_radix(&lcb, 2).unwrap();

    println!("mcb {:?} ({})", mcb, mcb_num);
    println!("lcb {:?} ({})", lcb, lcb_num);
    println!("Answer {}", lcb_num * mcb_num);
}

fn most_common_bits(input: Vec<&str>) -> String {
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
        .map(|total| if total > &(input.len() / 2) { "1" } else { "0" })
        .collect::<Vec<&str>>()
        .join("")
}

fn flip_bits(input: &str) -> String {
    input
        .chars()
        .map(|bit| if bit == '1' { "0" } else { "1" })
        .collect::<Vec<&str>>()
        .join("")
}
