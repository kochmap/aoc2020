use anyhow::Result;
use std::fs;
use std::collections::HashMap;

const SHINY_GOLD: &str = "shiny gold";

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let bags = shiny_gold_bag_contains(&input);
    println!("Shiny gold bag contains {:#?}", bags);
    Ok(())
}

fn bags_which_can_contain_shiny_golden_bag(rules: &str) -> Vec<String> {
    let mut bags: HashMap<String, Bag> = HashMap::new();
    for rule in rules.trim().lines() {
        let mut splitted = rule.trim_end_matches(".").split("contain");
        let name = splitted.next().unwrap().trim_end_matches(" bags ").to_string();
        let inside_bags = splitted.next().unwrap().trim().split(",").map(|str| str.trim()).flat_map(|bag_str| {
            if bag_str.contains("no other bags") {
                return Vec::new();
            }
            let split_at = bag_str.find(" ").unwrap();
            let name = bag_str[split_at + 1..bag_str.len()].trim_end_matches(" bags").trim_end_matches(" bag").to_string();
            let mut bags = Vec::new();
            bags.push(name.clone());
            bags
        }).collect();
        bags.insert(name.clone(), Bag { name, inner_bags: inside_bags });
    };
    let mut result_bags = Vec::new();
    for bag in bags.values() {
        if have_bag(&bags, bag) {
            result_bags.push(bag.name.clone());
        }
    };
    result_bags
}

fn have_bag(bags: &HashMap<String, Bag>, bag: &Bag) -> bool {
    bag.inner_bags.iter().find(|inner_bag| {
        if inner_bag.contains(SHINY_GOLD) {
            true
        } else if let Some(b) = bags.get(*inner_bag) {
            have_bag(bags, b)
        } else {
            false
        }
    }).is_some()
}

fn shiny_gold_bag_contains(rules: &str) -> usize {
    let mut bags: HashMap<String, Bag> = HashMap::new();
    for rule in rules.trim().lines() {
        let mut splitted = rule.trim_end_matches(".").split("contain");
        let name = splitted.next().unwrap().trim_end_matches(" bags ").to_string();
        let inside_bags = splitted.next().unwrap().trim().split(",").map(|str| str.trim()).flat_map(|bag_str| {
            if bag_str.contains("no other bags") {
                return Vec::new();
            }
            let split_at = bag_str.find(" ").unwrap();
            let number = bag_str[0..split_at].parse::<usize>().unwrap();
            let name = bag_str[split_at + 1..bag_str.len()].trim_end_matches(" bags").trim_end_matches(" bag").to_string();
            let mut bags = Vec::new();
            bags.push(name.clone());
            for _ in 1..number {
                bags.push(name.clone());
            }
            bags
        }).collect();
        bags.insert(name.clone(), Bag { name, inner_bags: inside_bags });
    };
    count_bags(&bags, &bags[SHINY_GOLD])
}

fn count_bags(bags: &HashMap<String, Bag>, bag: &Bag) -> usize {
    bag.inner_bags.iter().map(|inner_bag| {
        1 + bags.get(inner_bag).map_or_else(|| 0, |b| count_bags(bags, b))
    }).sum()
}

#[cfg(test)]
mod test {
    use crate::{bags_which_can_contain_shiny_golden_bag, shiny_gold_bag_contains};

    #[test]
    fn test_part1() {
        let test_rules = r"
light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.
        ";
        let mut expected_bags = vec!["bright white", "muted yellow", "dark orange", "light red"];
        expected_bags.sort();

        let mut bags = bags_which_can_contain_shiny_golden_bag(test_rules);
        bags.sort();

        assert_eq!(bags, expected_bags);
    }

    #[test]
    fn test_part2() {
        let test_rules = r"
shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.
        ";

        assert_eq!(shiny_gold_bag_contains(test_rules), 126);
    }
}

struct Bag {
    pub name: String,
    pub inner_bags: Vec<String>,
}