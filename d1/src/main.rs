use anyhow::anyhow;
use anyhow::Result;
use std::fs;

fn main() -> Result<()> {
    let input = fs::read_to_string("input2")?;
    let numbers: Vec<i32> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.parse().unwrap())
        .collect();
    let (n1, n2) = find_entries_sum(&numbers)?;
    println!("n1,n2: {},{}; n1*n2={}", n1, n2, n1 * n2);
    Ok(())
}

fn find_entries_sum(numbers: &[i32]) -> Result<(i32, i32)> {
    for (i, n1) in numbers.iter().enumerate() {
        if let Some(n2) = numbers[i + 1..].iter().find(|n2| n1 + *n2 == 2020) {
            return Ok((n1.clone(), n2.clone()));
        }
    }
    Err(anyhow!("Not found"))
}
