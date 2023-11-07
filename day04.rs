fn main() {
    println!("Opening door ...");
    let input = include_str!("04.txt");

    let mut contained_count = 0;
    let mut overlap_count = 0;
    for line in input.lines() {
        let nums = line
            .split(['-',','])
            .map(|num| num.parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        let contained = nums[0] <= nums[2] && nums[3] <= nums[1]
            || nums[2] <= nums[0] && nums[1] <= nums[3];
        let disjoint = nums[3] < nums[0] || nums[1] < nums[2];
        if contained { contained_count += 1; }
        if !disjoint { overlap_count += 1; }
    }
    // let sum = input.lines()
    //     .map(|line| line.split(['-',','])
    //         .map(|num| num.parse::<u32>().unwrap())
    //         .collect::<Vec<_>>())
    //     .fold()
    println!("Count of contained is {contained_count}");

    println!("Count of overlaps is {overlap_count}");

    println!("Goodbye");
}
