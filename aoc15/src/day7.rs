use crate::PUZZLES;
use crate::format_result;

use linkme::distributed_slice;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

const INPUT: &str = include_str!("../inputs/day7.txt");

#[derive(Eq, PartialEq, Clone, Copy)]
enum Func {
    Wire,
    Not,
    And,
    Or,
    Rshift,
    Lshift,
    Consts,
}

struct Component {
    valid: bool,
    output: u16,
    inputs: Vec<String>,
    func: Func,
}

impl Component {
    fn new(func: Func) -> Self {
        Self {
            valid: false,
            output: 0,
            inputs: vec![],
            func,
        }
    }

    fn new_constant(val: u16) -> Self {
        Self {
            valid: true,
            output: val,
            inputs: vec![],
            func: Func::Consts,
        }
    }

    fn add_input(&mut self, comp: String) {
        self.inputs.push(comp);
    }
}

type Context = Rc<HashMap<String, RefCell<Component>>>;

impl Component {
    fn get_output(&mut self, context: Context) -> u16 {
        match self.func {
            Func::Consts => self.output,
            Func::Wire => {
                if self.valid {
                    self.output
                } else {
                    assert!(self.inputs.len() == 1);
                    self.output = context
                        .get(&self.inputs[0])
                        .expect("not existed gate")
                        .borrow_mut()
                        .get_output(context.clone());
                    self.valid = true;
                    self.output
                }
            }
            Func::Not => {
                if self.valid {
                    self.output
                } else {
                    assert!(self.inputs.len() == 1);
                    self.output = !context
                        .get(&self.inputs[0])
                        .expect("not existed gate")
                        .borrow_mut()
                        .get_output(context.clone());
                    self.valid = true;
                    self.output
                }
            }
            Func::And => {
                if self.valid {
                    self.output
                } else {
                    assert!(self.inputs.len() == 2);
                    let input0 = context
                        .get(&self.inputs[0])
                        .expect("not existed gate")
                        .borrow_mut()
                        .get_output(context.clone());
                    let input1 = context
                        .get(&self.inputs[1])
                        .expect("not existed gate")
                        .borrow_mut()
                        .get_output(context.clone());
                    self.output = input0 & input1;
                    self.valid = true;
                    self.output
                }
            }
            Func::Or => {
                if self.valid {
                    self.output
                } else {
                    assert!(self.inputs.len() == 2);
                    let input0 = context
                        .get(&self.inputs[0])
                        .expect("not existed gate")
                        .borrow_mut()
                        .get_output(context.clone());
                    let input1 = context
                        .get(&self.inputs[1])
                        .expect("not existed gate")
                        .borrow_mut()
                        .get_output(context.clone());
                    self.output = input0 | input1;
                    self.valid = true;
                    self.output
                }
            }
            Func::Lshift => {
                if self.valid {
                    self.output
                } else {
                    assert!(self.inputs.len() == 2);
                    let input0 = context
                        .get(&self.inputs[0])
                        .expect("not existed gate")
                        .borrow_mut()
                        .get_output(context.clone());
                    let input1 = context
                        .get(&self.inputs[1])
                        .expect("not existed gate")
                        .borrow_mut()
                        .get_output(context.clone());
                    self.output = input0 << input1;
                    self.valid = true;
                    self.output
                }
            }
            Func::Rshift => {
                if self.valid {
                    self.output
                } else {
                    assert!(self.inputs.len() == 2);
                    let input0 = context
                        .get(&self.inputs[0])
                        .expect("not existed gate")
                        .borrow_mut()
                        .get_output(context.clone());
                    let input1 = context
                        .get(&self.inputs[1])
                        .expect("not existed gate")
                        .borrow_mut()
                        .get_output(context.clone());
                    self.output = input0 >> input1;
                    self.valid = true;
                    self.output
                }
            }
        }
    }
}

fn parse_line(line: &str, context: &mut HashMap<String, RefCell<Component>>) {
    let segs: Vec<&str> = line.split("->").collect();
    // input
    let inputs: Vec<&str> = segs[0].split_whitespace().collect();
    let output_name = segs[1].trim().to_string();
    let comp_name = segs[0].trim().to_string();
    match inputs.len() {
        1 => {
            // add input
            let input = inputs[0].to_string();
            if input.chars().next().unwrap().is_ascii_digit() {
                let constant: u16 = input.parse().expect("Not valid constant");
                context.insert(
                    comp_name.clone(),
                    RefCell::new(Component::new_constant(constant)),
                );
            }
            // add output wire
            let mut comp = Component::new(Func::Wire);
            comp.add_input(comp_name.clone());
            context
                .entry(output_name)
                .and_modify(|c| c.borrow_mut().add_input(comp_name))
                .or_insert(RefCell::new(comp));
        }
        2 => {
            // add input
            assert!(inputs[0] == "NOT");
            let input = inputs[1].to_string();
            let mut comp = Component::new(Func::Not);
            comp.add_input(input);
            context.insert(comp_name.clone(), RefCell::new(comp));
            // add output wire
            let mut comp = Component::new(Func::Wire);
            comp.add_input(comp_name.clone());
            context
                .entry(output_name)
                .and_modify(|c| c.borrow_mut().add_input(comp_name))
                .or_insert(RefCell::new(comp));
        }
        3 => {
            // add input
            let input0 = inputs[0].to_string();
            let func = match inputs[1] {
                "AND" => Func::And,
                "OR" => Func::Or,
                "LSHIFT" => Func::Lshift,
                "RSHIFT" => Func::Rshift,
                _ => panic!("Not valid func"),
            };
            let mut comp = Component::new(func);
            let input1 = inputs[2].to_string();
            if input1.chars().next().unwrap().is_ascii_digit() {
                let constant: u16 = input1.parse().expect("Not valid constant");
                context.insert(
                    input1.clone(),
                    RefCell::new(Component::new_constant(constant)),
                );
            }
            comp.add_input(input0);
            comp.add_input(input1);
            context.insert(comp_name.clone(), RefCell::new(comp));
            // add output wire
            let mut comp = Component::new(Func::Wire);
            comp.add_input(comp_name.clone());
            context
                .entry(output_name)
                .and_modify(|c| c.borrow_mut().add_input(comp_name))
                .or_insert(RefCell::new(comp));
        }
        _ => panic!("Not valid inputs"),
    }
}

fn construct_whole_circuit() -> Context {
    let mut context = HashMap::new();

    for line in INPUT.lines() {
        parse_line(line, &mut context);
    }

    Rc::new(context)
}

#[distributed_slice(PUZZLES)]
pub fn puzzle0() -> String {
    let context = construct_whole_circuit();

    let output = context
        .get("a")
        .expect("not found circuit a")
        .borrow_mut()
        .get_output(context.clone());

    format_result!(output);
}

#[distributed_slice(PUZZLES)]
pub fn puzzle1() -> String {
    let context = construct_whole_circuit();

    let output_a = context
        .get("a")
        .expect("not found circuit a")
        .borrow_mut()
        .get_output(context.clone());

    // reset
    for comp in context.values() {
        comp.borrow_mut().valid = false;
    }
    {
        let mut comp_b = context.get("b").expect("not found circuit a").borrow_mut();
        comp_b.valid = true;
        comp_b.output = output_a;
    }

    let output_a = context
        .get("a")
        .expect("not found circuit a")
        .borrow_mut()
        .get_output(context.clone());

    format_result!(output_a);
}
