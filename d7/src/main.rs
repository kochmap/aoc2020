use anyhow::Result;
use std::fs;
use std::collections::HashMap;

const SHINY_GOLD: &str = "shiny gold";

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let bags = bags_which_can_contain_shiny_golden_bag(&input);
    println!("Bags {:#?}, {} size", bags, bags.len());
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
            // let number = bag_str[0..split_at].parse::<usize>().unwrap();
            let name = bag_str[split_at + 1..bag_str.len()].trim_end_matches(" bags").trim_end_matches(" bag").to_string();
            let mut bags = Vec::new();
            bags.push(name.clone());
            /*            for _ in 0..number {
                            bags.push(name.clone());
                        }*/
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

#[cfg(test)]
mod test {
    use crate::bags_which_can_contain_shiny_golden_bag;

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
}

struct Bag {
    pub name: String,
    pub inner_bags: Vec<String>,
}