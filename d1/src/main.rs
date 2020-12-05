use anyhow::anyhow;
use anyhow::Result;
use std::fs;

fn main() -> Result<()> {
    let input = fs::read_to_string("input2.txt")?;
    let numbers: Vec<i32> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.parse().unwrap())
        .collect();
    // let (n1, n2) = find_two_entries_sum_2020(&numbers)?;
    // println!("n1,n2: {},{}; n1*n2={}", n1, n2, n1 * n2);
    let (n1, n2, n3) = find_three_entries_sum_2020(&numbers)?;
    println!("n1,n2,n3: {},{},{}; n1*n2*n3={}", n1, n2, n3, n1 * n2 * n3);
    Ok(())
}

fn find_two_entries_sum_2020(numbers: &[i32]) -> Result<(i32, i32)> {
    for (i, n1) in numbers.iter().enumerate() {
        if let Some(n2) = numbers[i + 1..].iter().find(|n2| n1 + *n2 == 2020) {
            return Ok((n1.clone(), n2.clone()));
        }
    }
    Err(anyhow!("Not found"))
}

fn find_three_entries_sum_2020(numbers: &[i32]) -> Result<(i32, i32, i32)> {
    for (i1, n1) in numbers.iter().enumerate() {
        for (i2, n2) in numbers[i1 + 1..].iter().enumerate() {
            if let Some(n3) = numbers[i2 + 1..].iter().find(|n3| n1 + *n2 + *n3 == 2020) {
                return Ok((n1.clone(), n2.clone(), n3.clone()));
            }
        }
    }
    Err(anyhow!("Not found"))
}
