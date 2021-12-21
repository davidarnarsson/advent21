use lib::read_file;

fn simulate(lines: &Vec<String>, generations: usize) {
    let mut deck: [u64; 9] = [0; 9];
    lines
        .first()
        .unwrap()
        .split(",")
        .map(|l| l.parse::<usize>().unwrap())
        .for_each(|n| deck[n] += 1);

    for _ in 0..generations {
        let new_fishes = deck[0];

        for i in 1..9 {
            deck[i - 1] = deck[i];
        }
        deck[6] = new_fishes + deck[6];
        deck[8] = new_fishes;
    }

    println!("{}", deck.to_vec().into_iter().sum::<u64>());
}


fn part_1(lines: &Vec<String>) {
    simulate(lines, 80);
}

fn part_2(lines: &Vec<String>) {
    simulate(lines, 256);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines = read_file("inputs/6.txt").unwrap();

    part_1(&lines);
    part_2(&lines);

    Ok(())
}
