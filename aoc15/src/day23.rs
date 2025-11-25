use crate::PUZZLES;
use crate::format_result;

use linkme::distributed_slice;

use std::str::FromStr;

enum Inst {
    Hlf(u8),
    Tpl(u8),
    Inc(u8),
    Jmp(isize),
    Jie(u8, isize),
    Jio(u8, isize),
}

impl FromStr for Inst {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        let op = parts.first().ok_or("Missing operation")?;

        let parse_reg = |r_str: &str| -> Result<u8, String> {
            match r_str.trim_end_matches(',') {
                "a" => Ok(0),
                "b" => Ok(1),
                _ => Err(format!("Invalid register: {}", r_str)),
            }
        };

        let parse_offset = |o_str: &str| -> Result<isize, String> {
            o_str.parse().map_err(|e| format!("Invalid offset: {}", e))
        };

        match *op {
            "hlf" => Ok(Inst::Hlf(parse_reg(
                parts.get(1).ok_or("Missing register")?,
            )?)),
            "tpl" => Ok(Inst::Tpl(parse_reg(
                parts.get(1).ok_or("Missing register")?,
            )?)),
            "inc" => Ok(Inst::Inc(parse_reg(
                parts.get(1).ok_or("Missing register")?,
            )?)),
            "jmp" => Ok(Inst::Jmp(parse_offset(
                parts.get(1).ok_or("Missing offset")?,
            )?)),
            "jie" => {
                let reg = parse_reg(parts.get(1).ok_or("Missing register")?)?;
                let offset = parse_offset(parts.get(2).ok_or("Missing offset")?)?;
                Ok(Inst::Jie(reg, offset))
            }
            "jio" => {
                let reg = parse_reg(parts.get(1).ok_or("Missing register")?)?;
                let offset = parse_offset(parts.get(2).ok_or("Missing offset")?)?;
                Ok(Inst::Jio(reg, offset))
            }
            _ => Err(format!("Unknown instruction: {}", op)),
        }
    }
}

struct Cpu {
    regs: [usize; 2],
    pc: isize,
}

impl Cpu {
    fn execute(&mut self, insts: &[Inst]) {
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

fn parse_input() -> Vec<Inst> {
    INPUT.lines().map(|line| line.parse().unwrap()).collect()
}

fn run_program(initial_a: usize) -> usize {
    let insts = parse_input();
    let mut cpu = Cpu {
        regs: [initial_a, 0],
        pc: 0,
    };
    cpu.execute(&insts);
    cpu.regs[1]
}

#[distributed_slice(PUZZLES)]
pub fn puzzle0() -> String {
    let ret = run_program(0);
    format_result!(ret)
}

#[distributed_slice(PUZZLES)]
pub fn puzzle1() -> String {
    let ret = run_program(1);
    format_result!(ret)
}
