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
    println!("Trees {}", trees(&lines));
    Ok(())
}

fn trees(lines: &[String]) -> usize {
    let (mut x, mut y, mut counter) = (0usize, 0usize, 0usize);
    // 3 right, 1 down
    while y < lines.len() - 1 {
        x += 3;
        y += 1;
        let mut line_chars = lines[y].chars().count();
        let mut chars = lines[y].chars();
        if chars.nth(x % line_chars).unwrap() == '#' {
            counter += 1;
        }
    }
    counter
}
