fn main() {
    let advent_of_code_door = "01.txt";
    println!("Starting {advent_of_code_door}...");
    let input = std::fs::read_to_string(advent_of_code_door)
        .expect("File: Something went wrong!");

    // let mut max = 0;
    // let mut current = 0;
    // for line in input.lines() {
    //     if line == "" {
    //         max = std::cmp::max(max, current);
    //         current = 0;
    //     } else {
    //         let ration: u32 = line.parse().expect("Non-u32 found on line!");
    //         current += ration;
    //     }
    // }
    // max = std::cmp::max(max, current);
    // println!("The maximum ration seems to be {}", max);

    let mut rations = vec![0];
    let mut accumulate = 0;
    for line in input.lines() {
        if line == "" {
            rations.push(accumulate);
            accumulate = 0;
        } else {
            let ration: u32 = line.parse().expect("Non-u32 found on line!");
            accumulate += ration;
        }
    }
    rations.sort_unstable();
    let the_sum: u32 = rations[rations.len()-3..].iter().sum();
    println!("The best ration is {}", rations[rations.len()-1]);
    println!("The sum of the best three rations seems to be {the_sum}");

    println!("Goodbye");
}
