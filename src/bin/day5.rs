use lib::read_file;
use std::error::Error;

fn parse(lines: &Vec<String>) -> Vec<(u16, u16, u16, u16)> {
    lines
        .into_iter()
        .map(|l| {
            l.split(" -> ")
                .flat_map(|pair| pair.split(",").map(|n| n.parse::<u16>().unwrap()))
                .collect()
        })
        .map(|l: Vec<u16>| (l[0], l[1], l[2], l[3]))
        .collect()
}

fn part_1(lines: &Vec<String>) {
    let parsed = parse(lines);

    // I couldn't be arsed with this...
}

fn part_2(lines: &Vec<String>) {}

pub fn main() -> Result<(), Box<dyn Error>> {
    println!("day 5");
    let lines = read_file("inputs/5.txt")?;
    

    part_1(&lines);
    part_2(&lines);

    Ok(())
}
