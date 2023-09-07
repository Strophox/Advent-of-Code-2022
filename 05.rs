use std::collections::LinkedList;

fn main() {
    println!("Opening door ...");
    let input = include_str!("05.txt");


    let mut crates: Vec<LinkedList<u8>> = (0..9).map(|_| LinkedList::new()).collect();
    let mut crates_9001: Vec<LinkedList<u8>> = (0..9).map(|_| LinkedList::new()).collect();
    for line in input.lines().map(|l| l.as_bytes()) {
        if line[0] != b'[' {
            break;
        }
        let mut i = 0;
        while 1+4*i < line.len() {
            if line[1+4*i] != b' ' {
                // println!("{}", line[1+4*i] as char);
                crates[i].push_front(line[1+4*i]);
                crates_9001[i].push_front(line[1+4*i]);
            }
            i += 1;
        }
    }
    for line in input.lines() {
        if line.len()==0 || line.as_bytes()[0] != b'm' {
            continue;
        }
        let items = line.split(' ')
            .map(|n| n.parse::<usize>().unwrap_or(0))
            .collect::<Vec<_>>();
        let amount = items[1];
        let src = items[3] - 1;
        let dst = items[5] - 1;
        for _ in 0..amount {
            let thing = crates[src].pop_back().unwrap();
            crates[dst].push_back(thing);
        }
        let x = crates_9001[src].len() - amount;
        // println!("9001: We are to do {amount}x {src}->{dst}, where Source stack is {:?} and destination {:?}", crates_9001[src], crates_9001[dst]);
        let stuff = &mut crates_9001[src].split_off(x);
        // println!("Stuff to move then looks like this: {:?}", stuff);
        crates_9001[dst].append(stuff);
        // println!("Result is then {:?}", crates_9001[dst]);
    }
    print!("It looks like this... ");
    for thing in crates {
        print!("{}", *thing.back().unwrap() as char);
    }
    println!(" ...do you see?");

    print!("The 9001 one looks like so... ");
    for thing in crates_9001 {
        print!("{}", *thing.back().unwrap() as char);
    }
    println!(" ...well?");

    println!("Goodbye");
}
