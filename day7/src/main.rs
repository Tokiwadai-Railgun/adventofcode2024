use std::fs; 

fn main() {
    let file_content = fs::read_to_string("src/entry.txt").unwrap();

    let values: Vec<(u16, Vec<u16>)>;
    for line in file_content.lines() {
        let split: Vec<_> = line.split(':').collect();
        let (number, value) = (split[0], split[1]);
    }

    let test = calculate(Vec::new());
}

fn calculate(numbers: Vec<u16>) -> u16 {
    32
}

// fn sum(values: Vec<u16>) -> u16 {}
