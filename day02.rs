fn main() {
    println!("Opening door ...");
    let input = include_str!("02.txt");

    let score: u32 = input.lines()
        .map(|line| jankenpon(parse(line)))
        .sum();
    println!("The score is {score}");

    let score_strategized: u32 = input.lines()
        .map(|line| jankenpon_strategized(parse(line)))
        .sum();
    println!("The score under the correct strategy is {score_strategized}");

    println!("Goodbye");
}

fn parse(line: &str) -> (u32, u32) {
    let (a, x) = (line.as_bytes()[0], line.as_bytes()[2]);
    ((a - b'A') as u32, (x - b'X') as u32)
}

fn jankenpon(hand: (u32, u32)) -> u32 {
    let shape = hand.1 + 1;
    let outcome = (hand.1 + 4 - hand.0) % 3 * 3;
    // let debug = ["rock","paper","scissors"]; println!("For {},{} shape is {shape}, outcome {outcome}", debug[hand1 as usize], debug[hand2 as usize]);
    shape + outcome
}

fn jankenpon_strategized((hand0, desired): (u32, u32)) -> u32 {
    jankenpon( (hand0, (hand0 + 2 + desired)%3) )
}
