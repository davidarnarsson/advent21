use lib::read_file;
use std::error::Error;

#[derive(Debug, PartialEq)]
enum Command {
    Up,
    Down,
    Forward,
}

impl std::str::FromStr for Command {
    fn from_str(val: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> {
        match val {
            "up" => Ok(Command::Up),
            "down" => Ok(Command::Down),
            "forward" => Ok(Command::Forward),
            _ => Err(()),
        }
    }
    type Err = ();
}

fn parse_commands(lines: &Vec<String>) -> Vec<(Command, i64)> {
    lines
        .into_iter()
        .map(|l| {
            let command_value: Vec<&str> = l.split(" ").collect();

            (
                command_value[0].parse::<Command>().unwrap(),
                command_value[1].parse::<i64>().unwrap(),
            )
        })
        .collect()
}

fn part_2(lines: &Vec<String>) -> Result<(), Box<dyn Error>> {
    let position = parse_commands(lines).into_iter().fold(
        (0, 0, 0),
        |(horz, depth, aim), (cmd, amt)| match cmd {
            Command::Up => (horz, depth, aim - amt),
            Command::Down => (horz, depth, aim + amt),
            Command::Forward => (horz + amt, depth + aim * amt, aim),
        },
    );

    println!(
        "Part 2: {:?} multipled is {}",
        position,
        position.0 * position.1
    );
    Ok(())
}

fn part_1(lines: &Vec<String>) -> Result<(), Box<dyn Error>> {
    let position = parse_commands(lines)
        .into_iter()
        .fold((0, 0), |(horz, depth), (cmd, amt)| match cmd {
            Command::Up => (horz, depth - amt),
            Command::Down => (horz, depth + amt),
            Command::Forward => (horz + amt, depth),
        });
    println!(
        "Part 1: {:?} multipled is {}",
        position,
        position.0 * position.1
    );

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("day 2");

    let lines = read_file("inputs/2.txt")?;
    part_1(&lines)?;
    part_2(&lines)?;

    Ok(())
}
