use std::fs;

use std::collections::HashMap;

fn main() {
    println!("Hello, world!");

    let filename = "example2";
    let filename2 = "full";
    //part1(filename);
    part2(filename);
    part2(filename2);
}

fn get_word_numbers(line: &str) -> HashMap<usize, u32> {
    let mut values = HashMap::new();

    let words = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut number: u32 = 1;
    for word in words {
        if let Some(idx) = line.find(word) {
            values.insert(idx, number);
        }
        number += 1;
    }

    values
}

fn part2(filename: &str) {
    if let Ok(contents) = fs::read_to_string(filename) {
        let mut all_numbers = Vec::new();

        let mut current_line_idx = 0;
        for line in contents.lines() {
            let line = line.to_ascii_lowercase();
            let mut numbers_this_line = Vec::new();

            let worded_numbers = get_word_numbers(&line);

            let characters: Vec<char> = line.chars().map(|a| a).collect();

            for i in 0..characters.len() {
                let c = characters[i];
                if c.is_ascii_digit() {
                    numbers_this_line.push(c.to_digit(10).unwrap());
                }
                if let Some(value) = worded_numbers.get(&i) {
                    println!("Letter start: {} = {}", c, value);
                    numbers_this_line.push(*value);
                }
            }

            let line_number = format!(
                "{}{}",
                numbers_this_line.first().unwrap(),
                numbers_this_line.last().unwrap()
            )
            .parse::<u32>()
            .unwrap();
            //println!("value: {}", line_number);
            all_numbers.push(line_number);
            if current_line_idx + 1 == 648 {
                println!("{}", line_number);
            }
            current_line_idx += 1;
        }

        let total: u32 = all_numbers.iter().sum();
        println!("Total: {}", total);
    }
}

fn part1(filename: &str) {
    if let Ok(contents) = fs::read_to_string(filename) {
        let mut all_numbers = Vec::new();

        for line in contents.lines() {
            let characters: Vec<char> = line.chars().map(|a| a).collect();
            let mut first: Option<u32> = None;
            let mut last: Option<u32> = first;
            characters.iter().for_each(|c| {
                if c.is_ascii_digit() {
                    if first.is_none() {
                        first = Some(c.to_digit(10).unwrap());
                    }
                    last = Some(c.to_digit(10).unwrap());
                }
            });

            if let (Some(first), Some(last)) = (first, last) {
                let number = format!("{}{}", first, last).parse::<u32>().unwrap();
                all_numbers.push(number);
            };
        }

        let sum: u32 = all_numbers.iter().sum();
        println!("Sum: {}", sum);
    }
}
