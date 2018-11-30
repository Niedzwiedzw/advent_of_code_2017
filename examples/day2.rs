use std::io::{ Result, Read };
use std::fs::File;

fn main() -> Result<()> {
    let mut file = File::open("data/day2.txt")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let lines: Vec<&str> = content.split("\r\n").collect();
    let lines: Vec<Vec<u32>> = lines
        .iter()
        .map(|e| e.split("\t")
            .map(|e| e.parse().unwrap())
            .collect()
        )
        .collect();

    let mut sum = 0;
    for line in lines {
        sum += line.iter().max().unwrap();
        sum -= line.iter().min().unwrap();
    }

    println!("{}", sum);

    Ok(())
}
