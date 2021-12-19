use lib::read_file;
use std::error::Error;

fn calc_increase(depths: &Vec<i64>) -> i64 {
    let former = &depths;
    let latter = &depths[1..];

    let depth_increase =
        former
            .into_iter()
            .zip(latter)
            .fold(0, |accum, (a, b)| if b > a { accum + 1 } else { accum });

    return depth_increase;
}

fn part_1(lines: Vec<String>) -> Result<(), Box<dyn Error>> {
    let depths: Vec<i64> = lines
        .iter()
        .map(|l| l.parse::<i64>().expect("Should parse!"))
        .collect::<Vec<i64>>();

    let depth_increase = calc_increase(&depths);

    println!("Part 1: Depth increased by {}", depth_increase);
    Ok(())
}

fn part_2(lines: Vec<String>) -> Result<(), Box<dyn Error>> {

    let measurements: Vec<i64> = lines
        .iter()
        .map(|l| l.parse::<i64>().expect("Should parse!"))
        .collect::<Vec<i64>>();
    let first = &measurements;
    let second = &measurements[1..];
    let third = &measurements[2..];

    let depths = third.iter().enumerate().map(|(i, v)| v + second[i] + first[i]).collect();

    let depth_increase = calc_increase(&depths);
    
    println!("Part 2: Depth increased by {}", depth_increase);
    Ok(())

}

fn main() -> Result<(), Box<dyn Error>> {
    println!("day 1");
    let lines = read_file("inputs/1.txt")?;
    part_1(lines.clone())?;
    part_2(lines.clone())
}
