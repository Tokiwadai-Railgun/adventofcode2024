use std::fs;
use regex::Regex;

fn main() {
    let input = fs::read_to_string("src/entry.txt").unwrap();
    let muls = find_correct_mul(&input);
    let muls_do = find_with_orders(&input);

    let mut total = 0;
    let mut total_do = 0;

    for mul_ in muls {
        let numbers = extract_number(mul_);
        total += mul(numbers[0], numbers[1]);
    }

    for mul_ in muls_do {
        let numbers = extract_number(mul_);
        total_do += mul(numbers[0], numbers[1]);
    }

    println!("Total : {total}");
    println!("Total part 2 : {total_do}");
}

// part 1 
fn find_correct_mul(input: &str) -> Vec<String> {
    // Use regex
    let re = Regex::new(r"mul\(([1-9]\d{0,2}),([1-9]\d{0,2})\)").unwrap();
    let mut response : Vec<String>= Vec::new();

    for match_ in re.find_iter(input) {
        response.push(String::from(match_.as_str()))
    }

    response
}

fn extract_number(input: String) -> Vec<i32> {
    let re = Regex::new(r"\d+").unwrap();
    re.find_iter(&input).map(|x| x.as_str().parse::<i32>().unwrap()).collect()
}

fn mul(x: i32, y: i32) -> i32 {
    x * y
}

// Par 2
fn find_with_orders(input: &str) ->Vec<String> {
    let re = Regex::new(r"\b(mul\(([1-9]\d{0,2}),([1-9]\d{0,2})\)|do\(\)|don't\(\))").unwrap();
    let mut result: Vec<String> = Vec::new();

    let mut do_ = true;
    // search for only the enabled ones
    for match_ in re.find_iter(input) {
        match match_.as_str() {
            "do()" => {do_ = true;},
            "don't()" => {do_ = false},
            other => {
                if do_ {
                    result.push(String::from(other));
                }             
            }
        }
    }

    result
}
