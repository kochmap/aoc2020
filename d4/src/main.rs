use anyhow::anyhow;
use anyhow::Result;
use std::fs;

const FIELDS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

fn main() -> Result<()> {
    let input = fs::read_to_string("input2.txt")?;
    let passports: Vec<String> = input.split("\n\n").map(|line| line.to_owned()).collect();
    let valid_passports = valid_passports(&passports);
    println!(
        "Valid passports {:#?}\n{}",
        valid_passports,
        valid_passports.len()
    );
    Ok(())
}

fn valid_passports<'a>(passports: &[String]) -> Vec<&String> {
    // byr (Birth Year)
    // iyr (Issue Year)
    // eyr (Expiration Year)
    // hgt (Height)
    // hcl (Hair Color)
    // ecl (Eye Color)
    // pid (Passport ID)
    // cid (Country ID) optional

    passports
        .iter()
        .filter(|p| FIELDS.iter().all(|f| p.contains(f)))
        .collect()
}
