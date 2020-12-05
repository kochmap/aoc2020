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
    println!("Trees {}", trees2(&lines));
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

fn trees2(lines: &[String]) -> usize {
    let mut counter = 1usize;
    // Right 1, down 1.
    // Right 3, down 1. (This is the slope you already checked.)
    // Right 5, down 1.
    // Right 7, down 1.
    // Right 1, down 2.
    let steps: [[usize; 2]; 5] = [[1, 1], [3, 1], [5, 1], [7, 1], [1, 2]];
    for [right, down] in steps.iter() {
        let (mut x, mut y, mut c) = (0usize, 0usize, 0usize);
        while y < lines.len() - down {
            x += right;
            y += down;
            let mut line_chars = lines[y].chars().count();
            let mut chars = lines[y].chars();
            if chars.nth(x % line_chars).unwrap() == '#' {
                c += 1;
            }
        }
        if c > 0 {
            counter *= c;
        }
    }
    counter
}
