use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut cpu = Cpu::new();
    fix_program(&mut cpu, &input);
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

fn run_until_exit_or_loop(cpu: &mut Cpu, program: &[Op]) -> bool {
    let mut pc_log = Vec::new();
    loop {
        if cpu.pc >= program.len() {
            return true;
        }
        if pc_log.contains(&cpu.pc) {
            return false;
        }
        pc_log.push(cpu.pc);
        cpu.execute(&program);
    }
}

fn fix_program(cpu: &mut Cpu, input: &str) {
    let mut program = Op::decode(input);
    let op_to_check: Vec<usize> = program.iter().enumerate().filter_map(|(i, op)| match op {
        Op::Jmp(_) => Some(i),
        Op::Nop(_) => Some(i),
        _ => None,
    }).collect();
    for i in op_to_check.iter() {
        let removed = program.remove(*i);
        let new_op = match removed {
            Op::Jmp(n) => Op::Nop(n),
            Op::Nop(n) => Op::Jmp(n),
            _ => panic!("Wrong op to replace in {}", i),
        };
        program.insert(*i, new_op);
        if run_until_exit_or_loop(cpu, &program) {
            break;
        }
        cpu.reset();
        program.remove(*i);
        program.insert(*i, removed);
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

    fn execute(&mut self, program: &[Op]) -> bool {
        if let Some(op) = program.get(self.pc) {
            match op {
                Op::Acc(n) => {
                    self.acc += n;
                    self.pc += 1;
                }
                Op::Jmp(n) => {
                    self.pc = (self.pc as i64 + n) as usize;
                }
                Op::Nop(_) => {
                    self.pc += 1;
                }
            }
            true
        } else {
            false
        }
    }

    fn reset(&mut self) {
        self.pc = 0;
        self.acc = 0;
    }
}

enum Op {
    Acc(i64),
    Jmp(i64),
    Nop(i64),
}

impl Op {
    fn decode(input: &str) -> Vec<Op> {
        input.lines().filter(|l| !l.is_empty()).map(|l| Op::decode_line(l)).collect()
    }

    fn decode_line(line: &str) -> Op {
        let mut splitted = line.split_ascii_whitespace();
        let op_raw = splitted.next().unwrap();
        let number = splitted.next().unwrap().parse::<i64>().unwrap();
        match op_raw {
            "acc" => Op::Acc(number),
            "jmp" => Op::Jmp(number),
            "nop" => Op::Nop(number),
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

    #[test]
    fn test_part2() {
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

        fix_program(&mut cpu, input);

        assert_eq!(cpu.acc, 8);
    }
}