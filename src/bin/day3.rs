use lib::read_file;
use std::cmp::{max, min};
use std::error::Error;

fn part_1(lines: &Vec<String>) {
    let (_numbers, counts, line_length) = count_numbers(lines);
    let last_bit = line_length - 1;

    let gamma: u32 = Vec::from(counts)
        .into_iter()
        .map(|i| min(1, max(0, i)) as u32)
        .enumerate()
        .fold(0, |accum, (i, v)| accum + (v * 1 << (last_bit - i)));

    let epsilon = gamma ^ ((1 << line_length) - 1);

    println!(
        "Part 1: gamma: {}, epsilon: {}, multiplied {}",
        gamma,
        epsilon,
        gamma * epsilon
    );
}

fn filter_count(
    numbers: Vec<u32>,
    line_length: usize,
    filter_cb: &dyn Fn(u32, u32) -> bool,
    tie_breaker: u32,
) -> u32 {
    let mut remainder = Vec::from(numbers);

    for i in 0..line_length {
        if remainder.len() == 1 {
            break;
        }
        let bit_shift = line_length - 1 - i;

        let bit = 1 << bit_shift;
        let foo = remainder.clone();
        let counted = foo
            .into_iter()
            .map(|n| if (n & bit) == bit { 1 } else { -1 })
            .sum();

        let mut count = min(1, max(0, counted)) as u32;

        if counted == 0 {
            count = tie_breaker;
        }

        remainder = remainder
            .into_iter()
            .filter(|n| {
                return filter_cb(n & (1 << bit_shift), count << bit_shift);
            })
            .collect();
    }

    return remainder.first().unwrap().clone();
}

fn part_2(lines: &Vec<String>) {
    let (numbers, _counts, line_length) = count_numbers(lines);

    let a = filter_count(numbers.clone(), line_length, &|a, b| a == b, 1);
    let b = filter_count(numbers.clone(), line_length, &|a, b| a != b, 1);

    println!("{} {} {}", a, b, a * b);
}

fn count_numbers(lines: &Vec<String>) -> (Vec<u32>, Vec<i32>, usize) {
    let numbers: Vec<u32> = lines
        .into_iter()
        .map(|i| u32::from_str_radix(i, 2).unwrap())
        .collect();

    let line_length = lines[0].len();
    let last_bit = line_length - 1;

    let mut counts: Vec<i32> = vec![0; line_length];
    for number in numbers.clone() {
        for i in 0..line_length {
            let bit = 1 << (last_bit - i);
            counts[i] += if (number & bit) == bit { 1 } else { -1 };
        }
    }

    return (numbers, counts, line_length);
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("day 3");

    let lines = read_file("inputs/3.txt")?;
    part_1(&lines);
    part_2(&lines);

    Ok(())
}
