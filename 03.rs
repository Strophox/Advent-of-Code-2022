use std::collections::HashSet;

fn main() {
    println!("Opening door ...");
    let input = include_str!("03.txt");

    let to_value = |l| if l <= b'Z' { l-b'A'+27 } else { l-b'a'+1 } as u32;

    let mut sum = 0;
    for line in input.lines() {
        let compartments = line.split_at(line.len()/2);
        let items1: HashSet<_> = compartments.0.bytes().collect();
        let items2: HashSet<_> = compartments.1.bytes().collect();
        let &character = items1
            .intersection(&items2)
            .into_iter().next().unwrap();
        sum += to_value(character);
    }

    println!("Sum of oddball item values is {sum}");

    let mut sum = 0;
    let mut lines = input.lines();
    while let Some(line1) = lines.next() {
        let line2 = lines.next().unwrap();
        let line3 = lines.next().unwrap();
        println!("{}, {}, {}", line1,line2,line3);
        for c in (b'A'..=b'Z').chain(b'a'..=b'z') {
            let character = c as char;
            if line1.contains(character)
            && line2.contains(character)
            && line3.contains(character) {
                sum += to_value(c);
                println!("Char was: {} with value {}", character,to_value(c));
                break;
            }
        }
    }

    println!("Sum of badge item values (or whatever) is {sum}");

    println!("Goodbye");
}
