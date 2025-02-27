use std::fs::read_to_string;

fn first_number(line: &str) -> Option<u32> {

    let mut digits = line.chars();

    while let Some(digit) = digits.next() {

        if digit.is_numeric() {
            return digit.to_digit(10);
        }

    };

    None

}

fn last_number(line: &str) -> Option<u32> {

    let mut digits = line.chars().rev();

    while let Some(digit) = digits.next() {

        if digit.is_numeric() {
            return digit.to_digit(10);
        }

    }

    None

}

fn main() {
    let input = match read_to_string("resources/input.txt") {
        Ok(v) => v,
        Err(_) => panic!("File not found")
    };

    let mut input = input.lines();

    let mut sum: u32 = 0;

    while let Some(line) = input.next() {
        
        let first = first_number(&line).unwrap();
        let last = last_number(&line).unwrap();

        let cal_value = 10*first + last;

        sum += cal_value;

    }

    println!("{sum}"); // 54561

}
