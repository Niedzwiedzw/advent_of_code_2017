// https://adventofcode.com/2017/day/1
use std::fs::File;
use std::io::{ Result, Read };


fn get_digits() -> Vec<u32> {
    let mut file = File::open("data/day1.txt").expect("File is provided.");
    let mut content = Vec::new();

    file.read_to_end(&mut content).expect("File is provided");
    content.iter()
        .map(|b| *b as char)
        .map(|d| d.to_digit(10 as u32).unwrap())
        .collect()
}

fn main() -> Result<()> {
    let digits = get_digits();
    let digit_cycle = digits.iter().chain(
        digits.iter().take(1)
    );
    let mut sum = 0;
    for (first, other) in digit_cycle.clone().zip(digit_cycle.skip(1)) {
        if first == other {
            sum += first;
        }
    }

    println!("{}", sum);
    Ok(())
}
