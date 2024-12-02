use std::fs;
fn main() {
    let entry = fs::read_to_string("src/entry.txt").unwrap().replace("\r", "");
    let mut total_correct = 0;
    let mut dampener_correct = 0;
    // without dampener
    for line in entry.split('\n') {
        if line.is_empty() { continue; }
        if is_safe(&line.split(" ").map(|x| x.parse::<i32>().unwrap()).collect()) {
            total_correct += 1;
        }

        if using_dampener(&line.split(" ").map(|x| x.parse::<i32>().unwrap()).collect()) {
            dampener_correct += 1;
        }
    }

    println!("Correct: {}", total_correct);
    println!("Dampener Correct : {}", dampener_correct);
}

fn is_safe(report: &Vec<i32>) -> bool{
    let mut signum = 0;
    let mut previous: Option<i32> = None;
    let mut correct = true;
    for i in report {
        if previous.is_none() {
            previous = Some(*i);
            continue;
        }

        let diff = i - previous.unwrap();
        if signum == 0 { signum = diff.signum()}

        // then check if the value is correct
        if signum != diff.signum() || diff.abs() > 3 || diff == 0{
            correct = false
        }

        previous = Some(*i);
    };

    correct
}


// Second part, simply bruteforcing here
fn using_dampener(report: &Vec<i32>) -> bool {
    if is_safe(report) { return true; }
    else {
        for i in 0..report.len() {
            let mut new_report = report.clone();
            new_report.remove(i);
            if is_safe(&new_report) { return true; }
        }
    }
    false
}
