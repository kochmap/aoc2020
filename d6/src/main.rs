use anyhow::anyhow;
use anyhow::Result;
use std::collections::{HashMap, HashSet};
use std::fs;

fn main() -> Result<()> {
    let input = fs::read_to_string("input2.txt")?;
    let groups_answers: Vec<String> = input.split("\n\n").map(|s| s.trim().to_owned()).collect();
    let answers_count = answers2(groups_answers);
    println!(
        "Answers count {:#?}, sum {}",
        answers_count,
        answers_count.iter().sum::<usize>()
    );
    Ok(())
}

fn answers(groups_answers: Vec<String>) -> Vec<usize> {
    groups_answers
        .iter()
        .map(|a| {
            let mut answers = HashSet::new();
            a.chars().filter(|c| c.is_alphabetic()).for_each(|c| {
                answers.insert(c);
            });
            answers.len()
        })
        .collect()
}

fn answers2(groups_answers: Vec<String>) -> Vec<usize> {
    groups_answers
        .iter()
        .map(|a| {
            let mut answers = HashSet::new();
            let mut all = 0usize;
            a.chars().filter(|c| c.is_alphabetic()).for_each(|c| {
                answers.insert(c.to_string());
            });
            answers.iter().for_each(|ans| {
                if a.lines().all(|line| line.contains(ans)) {
                    all += 1;
                };
            });
            all
        })
        .collect()
}
