use std::collections::BTreeSet;
use std::iter::FromIterator;

fn main() {
    println!("Opening door ...");
    let input = include_str!("06.txt");

    let do_thing = |n| {
        for i in n..input.len() {
            let current = &input.as_bytes()[i-n..i];
            if BTreeSet::from_iter(current.iter()).len() == n {
                return i;
            }
        }
        return 0;
    };

    println!("Packet marker found at {}.", do_thing(4));
    println!("Message marker found at {}.", do_thing(14));

    println!("Goodbye");
}
