use std::collections::BTreeSet;
use std::iter::FromIterator;

fn main() {
    println!("Opening door ...");
    let input = include_str!("06.txt");

    let mut index = 0;
    for i in 4..input.len() {
        let current = &input.as_bytes()[i-4..i];
        if BTreeSet::from_iter(current.iter()).len() == 4 {
            index = i;
            break;
        }
    }
    println!("Marker found at {index}.");

    let mut index = 0;
    for i in 14..input.len() {
        let current = &input.as_bytes()[i-14..i];
        if BTreeSet::from_iter(current.iter()).len() == 14 {
            index = i;
            break;
        }
    }
    println!("Message marker found at {index}.");

    println!("Goodbye");
}
