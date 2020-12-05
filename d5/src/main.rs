use anyhow::anyhow;
use anyhow::Result;
use std::fs;

fn main() -> Result<()> {
    let input = fs::read_to_string("input4.txt")?;
    let lines: Vec<String> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.trim().to_owned())
        .collect();
    let positions = positions(&lines);
    println!("Positions {:#?}", &positions);
    println!(
        "Max {:#?}",
        positions.iter().max_by_key(|p| p.position).unwrap()
    );
    Ok(())
}

fn positions(lines: &[String]) -> Vec<Position> {
    lines
        .iter()
        .map(|line| {
            let (mut row_low, mut row_high, mut column_low, mut column_high) =
                (0usize, 127usize, 0usize, 7usize);
            for char in line.chars() {
                match char {
                    'F' => row_high = row_low + (row_high - row_low) / 2,
                    'B' => row_low = row_high - (row_high - row_low) / 2,
                    'L' => column_high = column_low + (column_high - column_low) / 2,
                    'R' => column_low = column_high - (column_high - column_low) / 2,
                    _ => panic!("Wrong row {}, wrong char {}", line, char),
                };
            }
            let row = if row_low < row_high {
                row_low
            } else {
                row_high
            };
            let column = if column_low < column_high {
                column_low
            } else {
                column_high
            };
            Position {
                row,
                column,
                position: row * 8 + column,
            }
        })
        .collect()
}

#[derive(Debug)]
struct Position {
    pub row: usize,
    pub column: usize,
    pub position: usize,
}

// Start by considering the whole range, rows 0 through 127.
// F means to take the lower half, keeping rows 0 through 63.
// B means to take the upper half, keeping rows 32 through 63.
// F means to take the lower half, keeping rows 32 through 47.
// B means to take the upper half, keeping rows 40 through 47.
// B keeps rows 44 through 47.
// F keeps rows 44 through 45.
// The final F keeps the lower of the two, row 44.
