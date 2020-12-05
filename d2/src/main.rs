use anyhow::anyhow;
use anyhow::Result;
use std::fs;

fn main() -> Result<()> {
    let input = fs::read_to_string("input2.txt")?;
    let lines: Vec<String> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.to_owned())
        .collect();
    println!("Valid lines {}", valid_passwords2(&lines).len());
    Ok(())
}

fn valid_passwords(lines: &[String]) -> Vec<&String> {
    lines
        .iter()
        .filter(|line| {
            let parts: Vec<&str> = line.split_ascii_whitespace().collect();
            let part1: Vec<&str> = parts[0].split("-").collect();
            let (min, max): (usize, usize) = (part1[0].parse().unwrap(), part1[1].parse().unwrap());
            let letter: char = parts[1].chars().nth(0).unwrap();
            let len = parts[2].chars().filter(|c| letter == *c).count();
            println!("line {}, len {}", line, len);
            len >= min && len <= max
        })
        .collect()
}

fn valid_passwords2(lines: &[String]) -> Vec<&String> {
    lines
        .iter()
        .filter(|line| {
            let parts: Vec<&str> = line.split_ascii_whitespace().collect();
            let part1: Vec<&str> = parts[0].split("-").collect();
            let (first_position, second_position): (usize, usize) = (part1[0].parse().unwrap(), part1[1].parse().unwrap());
            let letter: char = parts[1].chars().nth(0).unwrap();
            let char1 = parts[2].chars().nth(first_position - 1).unwrap();
            let char2 = parts[2].chars().nth(second_position - 1).unwrap();
            println!("line {}, {}, {}", line, char1, char2);
            [char1, char2].iter().filter(|c| **c == letter).count() == 1
        })
        .collect()
}