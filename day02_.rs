fn main() {
    println!("Opening door ...");
    let input = include_str!("02.txt");

    let to_number_pair = |line: &[u8]| (line[0]-b'A', line[2]-b'X');
    let play_round = |(h0,h1): (u8,u8)| (h1+1 + (h1+4-h0)%3*3) as u32;

    let score: u32 = input
        .lines()
        .map(|line| line.as_bytes())
        .map(to_number_pair)
        .map(play_round)
        .sum();
    println!("The score is {score}.");

    let destrategize = |(h0,d1): (u8,u8)| (h0, (h0+2+d1)%3);

    let score_corrected: u32 = input
        .lines()
        .map(|line| line.as_bytes())
        .map(to_number_pair)
        .map(destrategize)
        .map(play_round)
        .sum();
    println!("The score under the correct strategy is {score_corrected}.");

    println!("Goodbye");
}
