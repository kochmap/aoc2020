use anyhow::anyhow;
use anyhow::Result;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fs;

const REQUIRED_FIELDS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
const ALL_FIELDS: [&str; 8] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"];

lazy_static! {
    static ref KEY_VALUE_REGEXP: Regex = Regex::new("(\\S+):(\\S+)").unwrap();
    static ref HCL_REGEXP: Regex = Regex::new("^#[0-9a-f]{6}$").unwrap();
    static ref ECL_REGEXP: Regex = Regex::new("^amb|blu|brn|gry|grn|hzl|oth$").unwrap();
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input2.txt")?;
    let passports: Vec<String> = input.split("\n\n").map(|line| line.to_owned()).collect();
    let valid_passports = valid_passports2(&passports);
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
        .filter(|p| REQUIRED_FIELDS.iter().all(|f| p.contains(f)))
        .collect()
}

fn valid_passports2<'a>(passports: &[String]) -> Vec<PassportValidation> {
    passports
        .iter()
        .map(|p| {
            let key_values = KEY_VALUE_REGEXP.captures_iter(p);
            let mut map = HashMap::new();
            for captures in key_values {
                let c1 = (&captures[1]).to_string();
                let c2 = (&captures[2]).to_string();
                map.insert(c1, c2);
            }
            let validation = PassportValidation::validate(&map);
            println!(
                "### passport ###\n{}\n### serialized ###\n{:#?}\n### validation ###\n{:#?}\n\n\n",
                p, map, validation
            );
            validation
        })
        .filter(|validation| {
            validation.valid()
        })
        .collect()
}

trait PassportVerifier {
    fn number_assertion<F: Fn(usize) -> bool>(&self, key: &str, p: F) -> bool;
    fn assertion<F: Fn(&str) -> bool>(&self, key: &str, p: F) -> bool;
    fn get_owned_string(&self, key: &str) -> String;
}

impl PassportVerifier for HashMap<String, String> {
    fn number_assertion<F: Fn(usize) -> bool>(&self, key: &str, p: F) -> bool {
        self.assertion(key, |value| {
            if let Ok(num) = value.parse() {
                p(num)
            } else {
                false
            }
        })
    }

    fn assertion<F: Fn(&str) -> bool>(&self, key: &str, p: F) -> bool {
        self.get(key).map_or_else(|| false, |value| p(value))
    }

    fn get_owned_string(&self, key: &str) -> String {
        self.get(key).map(|k| k.clone()).unwrap_or(String::new())
    }
}

#[derive(Debug)]
struct PassportValidation {
    pub byr: String,
    pub byr_check: bool,
    pub iyr: String,
    pub iyr_check: bool,
    pub eyr: String,
    pub eyr_check: bool,
    pub hgt: String,
    pub hgt_check: bool,
    pub hcl: String,
    pub hcl_check: bool,
    pub ecl: String,
    pub ecl_check: bool,
    pub pid: String,
    pub pid_check: bool,
    pub unknown_field_check: bool,
}

impl PassportValidation {
    fn validate(map: &HashMap<String, String>) -> PassportValidation {
        // byr (Birth Year) - four digits; at least 1920 and at most 2002.
        // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
        // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
        // hgt (Height) - a number followed by either cm or in:
        // - If cm, the number must be at least 150 and at most 193.
        // - If in, the number must be at least 59 and at most 76.
        // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
        // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
        // pid (Passport ID) - a nine-digit number, including leading zeroes.
        // cid (Country ID) - ignored, missing or not.
        let byr_check = map.number_assertion("byr", |num| num >= 1920 && num <= 2002);
        let iyr_check = map.number_assertion("iyr", |num| num >= 2010 && num <= 2020);
        let eyr_check = map.number_assertion("eyr", |num| num >= 2020 && num <= 2030);
        let hgt_check = map.assertion("hgt", |value| {
            if value.contains("cm") {
                value.strip_suffix("cm").map(|n| {
                    n.parse::<usize>()
                        .map(|v| v >= 150 && v <= 193)
                        .unwrap_or(false)
                })
            } else if value.contains("in") {
                value.strip_suffix("in").map(|n| {
                    n.parse::<usize>()
                        .map(|v| v >= 59 && v <= 76)
                        .unwrap_or(false)
                })
            } else {
                None
            }
            .unwrap_or(false)
        });
        let hcl_check = map.assertion("hcl", |value| HCL_REGEXP.is_match(value));
        let ecl_check = map.assertion("ecl", |value| ECL_REGEXP.is_match(value));
        let pid_check = map.assertion("pid", |value| {
            value.len() < 10 && value.chars().all(|c| c.is_digit(10))
        });
        let unknown_field_check = map.keys().all(|k| ALL_FIELDS.contains(&k.as_str()));
        PassportValidation {
            byr: map.get_owned_string("byr"),
            byr_check,
            iyr: map.get_owned_string("iyr"),
            iyr_check,
            eyr: map.get_owned_string("eyr"),
            eyr_check,
            hgt: map.get_owned_string("hgt"),
            hgt_check,
            hcl: map.get_owned_string("hcl"),
            hcl_check,
            ecl: map.get_owned_string("ecl"),
            ecl_check,
            pid: map.get_owned_string("pid"),
            pid_check,
            unknown_field_check,
        }
    }

    fn valid(&self) -> bool {
        self.byr_check
            && self.iyr_check
            && self.eyr_check
            && self.hgt_check
            && self.hcl_check
            && self.ecl_check
            && self.pid_check
    }
}
