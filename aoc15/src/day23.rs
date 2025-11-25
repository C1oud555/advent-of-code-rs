use core::panic;

use crate::PUZZLES;
use crate::format_result;

use linkme::distributed_slice;

enum Inst {
    Hlf(u8),
    Tpl(u8),
    Inc(u8),
    Jmp(isize),
    Jie(u8, isize),
    Jio(u8, isize),
}

struct Cpu {
    regs: [usize; 2],
    pc: isize,
}

impl Cpu {
    fn execute(&mut self, insts: Vec<Inst>) {
        while (self.pc as usize) < insts.len() {
            let inst = &insts[self.pc as usize];
            match inst {
                Inst::Hlf(reg) => {
                    self.regs[*reg as usize] /= 2;
                    self.pc += 1;
                }
                Inst::Tpl(reg) => {
                    self.regs[*reg as usize] *= 3;
                    self.pc += 1;
                }
                Inst::Inc(reg) => {
                    self.regs[*reg as usize] += 1;
                    self.pc += 1;
                }
                Inst::Jmp(offset) => self.pc += offset,
                Inst::Jie(reg, offset) => {
                    if self.regs[*reg as usize].is_multiple_of(2) {
                        self.pc += offset
                    } else {
                        self.pc += 1;
                    }
                }
                Inst::Jio(reg, offset) => {
                    if self.regs[*reg as usize] == 1 {
                        self.pc += offset
                    } else {
                        self.pc += 1;
                    }
                }
            }
        }
    }
}

const INPUT: &str = include_str!("../inputs/day23.txt");

fn str_reg_num(i: &str) -> u8 {
    if i.starts_with("a") {
        0
    } else if i.starts_with("b") {
        1
    } else {
        panic!("not valid reg")
    }
}

fn parse_inst(line: &str) -> Inst {
    let inst: Vec<&str> = line.split_whitespace().collect();
    if inst.len() == 2 {
        match inst[0] {
            "hlf" => Inst::Hlf(str_reg_num(inst[1])),
            "tpl" => Inst::Tpl(str_reg_num(inst[1])),
            "inc" => Inst::Inc(str_reg_num(inst[1])),
            "jmp" => Inst::Jmp(inst[1].parse::<isize>().expect("not valid jump")),
            _ => panic!("invalid instruction"),
        }
    } else if inst.len() == 3 {
        match inst[0] {
            "jie" => Inst::Jie(str_reg_num(inst[1]), inst[2].parse::<isize>().unwrap()),
            "jio" => Inst::Jio(str_reg_num(inst[1]), inst[2].parse::<isize>().unwrap()),
            _ => panic!("invalid instruction"),
        }
    } else {
        panic!("not valid len");
    }
}

fn parse_input() -> Vec<Inst> {
    let mut ret = Vec::new();
    for line in INPUT.lines() {
        ret.push(parse_inst(line));
    }

    ret
}

#[distributed_slice(PUZZLES)]
pub fn puzzle0() -> String {
    let insts = parse_input();
    let mut cpu = Cpu {
        regs: [0, 0],
        pc: 0,
    };
    cpu.execute(insts);
    let ret = cpu.regs[1];

    format_result!(ret)
}

#[distributed_slice(PUZZLES)]
pub fn puzzle1() -> String {
    let insts = parse_input();
    let mut cpu = Cpu {
        regs: [1, 0],
        pc: 0,
    };
    cpu.execute(insts);
    let ret = cpu.regs[1];

    format_result!(ret)
}
