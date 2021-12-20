use lib::read_file;
use std::collections::VecDeque;
use std::error::Error;
use std::fmt::Write;

#[derive(Debug, Clone)]
struct Board {
    cells: [i16; 25],
    marked: [bool; 25],
}

impl Board {
    fn new(lines: &[String]) -> Board {
        Board {
            cells: lines
                .into_iter()
                .flat_map(|l| {
                    l.split(" ")
                        .filter(|i| i.len() > 0)
                        .map(|i| i.parse::<i16>().unwrap())
                })
                .collect::<Vec<i16>>()
                .try_into()
                .unwrap_or_else(|v: Vec<i16>| {
                    panic!("Expected array of len {} but got {}", 25, v.len())
                }),
            marked: [false; 25],
        }
    }

    fn draw(&mut self, number: i16) {
        for i in 0..25 {
            if self.cells[i] == number {
                self.marked[i] = true;
            }
        }
    }

    fn score(&self) -> i16 {
        return self
            .cells
            .iter()
            .enumerate()
            .filter(|(i, _)| !self.marked[*i])
            .map(|(_, x)| x)
            .sum();
    }

    fn has_bingo(&self) -> bool {
        //let diagonal_from_left = (0..5).map(|n| n * 6).all(|n| self.marked[n]);
        //let diagonal_from_right = (0..5).map(|n| n * 4).all(|n| self.marked[n]);
        let horizontal = (0..5).any(|n| (0..5).all(|m| self.marked[n * 5 + m]));
        let vertical = (0..5).any(|n| (0..5).all(|m| self.marked[n + 5 * m]));

        return horizontal || vertical;
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        fmt.write_char('\n')?;
        for i in 0..5 {
            for j in 0..5 {
                let index = i * 5 + j;
                let mut cell = format!("{}", self.cells[index]);
                if self.marked[index] {
                    cell = format!("[{}]", cell);
                }

                fmt.write_fmt(format_args!("\t{}", cell))?;
            }
            fmt.write_char('\n')?;
        }
        Ok(())
    }
}

fn parse_bingo(lines: &Vec<String>) -> (VecDeque<i16>, Vec<Board>) {
    let draw: VecDeque<i16> = lines
        .first()
        .unwrap()
        .split(",")
        .map(|n| n.parse::<i16>().unwrap())
        .collect();

    let mut slice = &lines[1..];

    let mut cards = Vec::new();
    loop {
        if slice.len() == 0 {
            break;
        }

        let card = &slice[0..6];

        cards.push(Board::new(card));

        slice = &slice[6..];
    }

    return (draw, cards);
}

fn part_1(lines: &Vec<String>) {
    let (mut draw, mut cards) = parse_bingo(lines);

    let mut bingo_card = None;
    loop {
        let number = draw.pop_front().unwrap();
        println!("Drew {}", number);
        for card in cards.iter_mut() {
            card.draw(number);
            if card.has_bingo() {
                bingo_card = Some(card.clone());
            }
        }

        if let Some(c) = bingo_card {
            println!("Score: {}, {}", c.score() * number, c);
            break;
        }
    }
}

fn part_2(lines: &Vec<String>) {
    let (mut draw, mut cards) = parse_bingo(lines);

    let mut winning_score: Option<i32> = None;
    loop {
        match draw.pop_front() {
            Some(number) => {
                for card in cards.iter_mut() {
                    card.draw(number);
                    if card.has_bingo() {
                        winning_score = Some((card.score() as i32) * (number as i32));
                    }
                }
                cards = cards.into_iter().filter(|c| !c.has_bingo()).collect();
            }
            None => {
                if let Some(c) = winning_score {
                    println!("Score: {}", c);
                    break;
                }
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("day 3");

    let lines = read_file("inputs/4.txt")?;
    part_1(&lines);
    part_2(&lines);

    Ok(())
}
