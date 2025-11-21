use crate::PUZZLES;
use crate::format_result;

use linkme::distributed_slice;
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, line_ending, u16 as nom_u16},
    combinator::{map, map_res},
    multi::many1,
    sequence::separated_pair,
};
use rustc_hash::FxHashMap;

const INPUT: &str = include_str!("../inputs/day7.txt");

type Wire = u16;

#[derive(Debug, Clone, Copy)]
enum Input {
    Wire(Wire),
    Signal(u16),
}

impl Input {
    fn value(&self, signals: &[Option<u16>]) -> Option<u16> {
        match self {
            Input::Wire(wire) => signals[*wire as usize],
            Input::Signal(signal) => Some(*signal),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Gate {
    Set(Input),
    And(Input, Input),
    Or(Input, Input),
    LShift(Input, u16),
    RShift(Input, u16),
    Not(Input),
}

impl Gate {
    fn eval(&self, signals: &[Option<u16>]) -> Option<u16> {
        match self {
            Gate::Set(a) => a.value(signals),
            Gate::And(a, b) => {
                if let (Some(a), Some(b)) = (a.value(signals), b.value(signals)) {
                    Some(a & b)
                } else {
                    None
                }
            }
            Gate::Or(a, b) => {
                if let (Some(a), Some(b)) = (a.value(signals), b.value(signals)) {
                    Some(a | b)
                } else {
                    None
                }
            }
            Gate::LShift(a, b) => a.value(signals).map(|a| a << *b),
            Gate::RShift(a, b) => a.value(signals).map(|a| a >> *b),
            Gate::Not(a) => a.value(signals).map(|a| !a),
        }
    }
}

enum ParsedInput<'a> {
    Wire(&'a str),
    Signal(u16),
}

enum ParsedGate<'a> {
    Set(ParsedInput<'a>, &'a str),
    And(ParsedInput<'a>, ParsedInput<'a>, &'a str),
    Or(ParsedInput<'a>, ParsedInput<'a>, &'a str),
    LShift(ParsedInput<'a>, u16, &'a str),
    RShift(ParsedInput<'a>, u16, &'a str),
    Not(ParsedInput<'a>, &'a str),
}

fn parse_input(i: &str) -> IResult<&str, ParsedInput<'_>> {
    alt((
        map(nom_u16, ParsedInput::Signal),
        map(alpha1, ParsedInput::Wire),
    ))
    .parse(i)
}

fn parse_gate(i: &str) -> IResult<&str, ParsedGate<'_>> {
    let parse_set = map(
        separated_pair(parse_input, tag(" -> "), alpha1),
        |(input, wire)| ParsedGate::Set(input, wire),
    );
    let parse_not = map(
        separated_pair((tag("NOT "), parse_input), tag(" -> "), alpha1),
        |((_, input), wire)| ParsedGate::Not(input, wire),
    );
    let parse_op = map(
        separated_pair(
            (
                parse_input,
                alt((
                    map(tag(" AND "), |_| "AND"),
                    map(tag(" OR "), |_| "OR"),
                    map(tag(" LSHIFT "), |_| "LSHIFT"),
                    map(tag(" RSHIFT "), |_| "RSHIFT"),
                )),
                parse_input,
            ),
            tag(" -> "),
            alpha1,
        ),
        |((in1, op, in2), wire)| match op {
            "AND" => ParsedGate::And(in1, in2, wire),
            "OR" => ParsedGate::Or(in1, in2, wire),
            "LSHIFT" => {
                if let ParsedInput::Signal(val) = in2 {
                    ParsedGate::LShift(in1, val, wire)
                } else {
                    panic!("LSHIFT requires a signal as the second argument");
                }
            }
            "RSHIFT" => {
                if let ParsedInput::Signal(val) = in2 {
                    ParsedGate::RShift(in1, val, wire)
                } else {
                    panic!("RSHIFT requires a signal as the second argument");
                }
            }
            _ => unreachable!(),
        },
    );

    alt((parse_not, parse_op, parse_set)).parse(i)
}

fn parse_circuit(i: &str) -> (Vec<(Gate, Wire)>, FxHashMap<String, Wire>) {
    let (_rem, parsed_gates) = many1(map_res((parse_gate, line_ending), |(g, _)| {
        Ok::<_, &str>(g)
    }))
    .parse(i)
    .unwrap();

    let mut wires = FxHashMap::default();
    let mut next_wire = 0;
    let mut gates = vec![];

    let get_wire = |s: &str, wires: &mut FxHashMap<String, Wire>, next_wire: &mut u16| {
        *wires.entry(s.to_string()).or_insert_with(|| {
            let ret = *next_wire;
            *next_wire += 1;
            ret
        })
    };

    let to_input = |parsed: ParsedInput,
                    wires: &mut FxHashMap<String, Wire>,
                    next_wire: &mut u16| match parsed {
        ParsedInput::Signal(s) => Input::Signal(s),
        ParsedInput::Wire(w) => Input::Wire(get_wire(w, wires, next_wire)),
    };

    for parsed_gate in parsed_gates {
        match parsed_gate {
            ParsedGate::Set(input, wire) => {
                let input = to_input(input, &mut wires, &mut next_wire);
                let wire = get_wire(wire, &mut wires, &mut next_wire);
                gates.push((Gate::Set(input), wire));
            }
            ParsedGate::And(in1, in2, wire) => {
                let in1 = to_input(in1, &mut wires, &mut next_wire);
                let in2 = to_input(in2, &mut wires, &mut next_wire);
                let wire = get_wire(wire, &mut wires, &mut next_wire);
                gates.push((Gate::And(in1, in2), wire));
            }
            ParsedGate::Or(in1, in2, wire) => {
                let in1 = to_input(in1, &mut wires, &mut next_wire);
                let in2 = to_input(in2, &mut wires, &mut next_wire);
                let wire = get_wire(wire, &mut wires, &mut next_wire);
                gates.push((Gate::Or(in1, in2), wire));
            }
            ParsedGate::LShift(input, val, wire) => {
                let input = to_input(input, &mut wires, &mut next_wire);
                let wire = get_wire(wire, &mut wires, &mut next_wire);
                gates.push((Gate::LShift(input, val), wire));
            }
            ParsedGate::RShift(input, val, wire) => {
                let input = to_input(input, &mut wires, &mut next_wire);
                let wire = get_wire(wire, &mut wires, &mut next_wire);
                gates.push((Gate::RShift(input, val), wire));
            }
            ParsedGate::Not(input, wire) => {
                let input = to_input(input, &mut wires, &mut next_wire);
                let wire = get_wire(wire, &mut wires, &mut next_wire);
                gates.push((Gate::Not(input), wire));
            }
        }
    }

    (gates, wires)
}

struct Evaluator {
    gates: Vec<(Gate, Wire)>,
    signals: Vec<Option<u16>>,
    num_wires: u16,
}

impl Evaluator {
    fn new(gates: Vec<(Gate, Wire)>, num_wires: u16) -> Self {
        Self {
            gates,
            signals: vec![None; num_wires as usize],
            num_wires,
        }
    }

    fn eval(&mut self, wire: Wire) -> u16 {
        while self.signals[wire as usize].is_none() {
            for (gate, wire) in &self.gates {
                if self.signals[*wire as usize].is_none() {
                    self.signals[*wire as usize] = gate.eval(&self.signals);
                }
            }
        }
        self.signals[wire as usize].unwrap()
    }

    fn reset(&mut self) {
        self.signals = vec![None; self.num_wires as usize];
    }
}

#[distributed_slice(PUZZLES)]
pub fn puzzle0() -> String {
    let (gates, wires) = parse_circuit(INPUT);
    let mut evaluator = Evaluator::new(gates, wires.len() as u16);
    let a_wire = wires["a"];

    let output = evaluator.eval(a_wire);
    format_result!(output);
}

#[distributed_slice(PUZZLES)]
pub fn puzzle1() -> String {
    let (gates, wires) = parse_circuit(INPUT);
    let mut evaluator = Evaluator::new(gates, wires.len() as u16);
    let a_wire = wires["a"];
    let b_wire = wires["b"];

    let a_val = evaluator.eval(a_wire);

    evaluator.reset();

    evaluator.signals[b_wire as usize] = Some(a_val);
    let output = evaluator.eval(a_wire);
    format_result!(output);
}
