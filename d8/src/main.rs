use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut cpu = Cpu::new();
    run_until_loop(&mut cpu, &input);
    println!("acc {}", cpu.acc)
}

fn run_until_loop(cpu: &mut Cpu, input: &str) {
    let program = Op::decode(input);
    let mut pc_log = Vec::new();
    loop {
        if pc_log.contains(&cpu.pc) {
            break;
        }
        pc_log.push(cpu.pc);
        cpu.execute(&program);
    }
}

struct Cpu {
    pub pc: usize,
    pub acc: i64,
}

impl Cpu {
    fn new() -> Cpu {
        Cpu {
            pc: 0,
            acc: 0,
        }
    }

    fn execute(&mut self, program: &[Op]) {
        match program[self.pc] {
            Op::Acc(n) => {
                self.acc += n;
                self.pc += 1;
            }
            Op::Jmp(n) => {
                self.pc = (self.pc as i64 + n) as usize;
            }
            Op::Nop => {
                self.pc += 1;
            }
        }
    }
}

enum Op {
    Acc(i64),
    Jmp(i64),
    Nop,
}

impl Op {
    fn decode(input: &str) -> Vec<Op> {
        input.lines().filter(|l| !l.is_empty()).map(|l| Op::decode_line(l)).collect()
    }

    fn decode_line(line: &str) -> Op {
        let mut splitted = line.split_ascii_whitespace();
        match splitted.next().unwrap() {
            "acc" => Op::Acc(splitted.next().unwrap().parse::<i64>().unwrap()),
            "jmp" => Op::Jmp(splitted.next().unwrap().parse::<i64>().unwrap()),
            "nop" => Op::Nop,
            _ => panic!("Not known op {}", line),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_part1() {
        let mut cpu = Cpu::new();
        let input = r"
nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6
";

        run_until_loop(&mut cpu, input);

        assert_eq!(cpu.acc, 5);
    }
}