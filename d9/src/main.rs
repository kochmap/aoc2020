use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("{}", find_wrong_number(&input, 25))
}

fn find_wrong_number(input: &str, previous: usize) -> usize {
    let numbers: Vec<usize> = input.lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.parse::<usize>().unwrap())
        .collect();
    let mut wrong: Option<usize> = None;
    for (i, number) in numbers.iter().enumerate() {
        if i < previous {
            continue;
        }
        let start = if i <= previous {
            0
        } else {
            i - previous
        };
        if !sum_exists(*number, &numbers[start..i]) {
            wrong = Some(*number);
            break;
        }
    };
    wrong.unwrap()
}

fn sum_exists(number: usize, numbers: &[usize]) -> bool {
    let mut sums: Vec<usize> = Vec::new();
    for (i1, n1) in numbers.iter().enumerate() {
        for (i2, n2) in numbers.iter().enumerate() {
            if i1 == i2 {
                continue;
            }
            sums.push(n1 + n2);
        }
    }
    sums.contains(&number)
}

#[cfg(test)]
mod test {
    use crate::find_wrong_number;

    #[test]
    fn test1() {
        let input = r"
35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576
        ";

        assert_eq!(find_wrong_number(input, 5), 127)
    }
}