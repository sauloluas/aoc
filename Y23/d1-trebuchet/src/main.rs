use std::fs::read_to_string;

const NUMBERS: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn contains_spelled_number(buf: &str) -> Option<&str> {

    for number in NUMBERS {
        if buf.contains(number) {
            return Some(number);
        }
    };

    None

}

fn spelled_number_to_int(number: &str) -> Option<u32> {
    
    match number {
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        _ => None
    }

}

fn first_number(line: &str) -> Option<u32> {

    let mut digits = line.chars();

    let mut buf = String::new();


    while let Some(digit) = digits.next() {

        if digit.is_numeric() {

            return digit.to_digit(10);

        } else {

            buf.push(digit);

            if let Some(number) = contains_spelled_number(&buf) {
                return spelled_number_to_int(number);
            } 

        }



    };

    None

}



fn last_number(line: &str) -> Option<u32> {

    let mut digits = line.chars().rev();

    let mut buf = String::new();

    while let Some(digit) = digits.next() {

        if digit.is_numeric() {
            return digit.to_digit(10);
        } else {

            buf.insert(0, digit);

            if let Some(number) = contains_spelled_number(&buf) {
                return spelled_number_to_int(number);
            }

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

    // let mut i: u32 = 0;

    while let Some(line) = input.next() {
        
        let first = first_number(&line).unwrap();
        let last = last_number(&line).unwrap();

        let cal_value = 10*first + last;

        sum += cal_value;

        // i += 1;

        // if i > 10 {
        //     break
        // };

    }

    println!("{sum}"); // 54561

}
