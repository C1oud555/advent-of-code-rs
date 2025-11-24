use crate::PUZZLES;
use crate::format_result;
use linkme::distributed_slice;
use nom::{
    IResult,
    Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, line_ending, u16 as nom_u16},
    combinator::{map, map_res},
    multi::many1,
    sequence::{preceded, separated_pair}, // Correctly import separated_pair
};
use rustc_hash::FxHashMap;
use std::collections::VecDeque;

const INPUT: &str = include_str!("../inputs/day7.txt");

type WireId = u16;

#[derive(Debug, Clone, Copy)]
enum Input {
    Wire(WireId),
    Signal(u16),
}

impl Input {
    fn value(&self, signals: &[Option<u16>]) -> Option<u16> {
        match self {
            Input::Wire(wire_id) => signals[*wire_id as usize],
            Input::Signal(signal) => Some(*signal),
        }
    }

    fn wire_ids(&self) -> Vec<WireId> {
        if let Input::Wire(id) = self {
            vec![*id]
        } else {
            vec![]
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

    fn input_wire_ids(&self) -> Vec<WireId> {
        match self {
            Gate::Set(a) => a.wire_ids(),
            Gate::And(a, b) => a.wire_ids().into_iter().chain(b.wire_ids()).collect(),
            Gate::Or(a, b) => a.wire_ids().into_iter().chain(b.wire_ids()).collect(),
            Gate::LShift(a, _) => a.wire_ids(),
            Gate::RShift(a, _) => a.wire_ids(),
            Gate::Not(a) => a.wire_ids(),
        }
    }
}

enum ParsedInput<'a> {
    Wire(&'a str),
    Signal(u16),
}

enum ParsedGate<'a> {
    Set(ParsedInput<'a>),
    And(ParsedInput<'a>, ParsedInput<'a>),
    Or(ParsedInput<'a>, ParsedInput<'a>),
    LShift(ParsedInput<'a>, u16),
    RShift(ParsedInput<'a>, u16),
    Not(ParsedInput<'a>),
    OutputWire(&'a str, Box<ParsedGate<'a>>),
}

fn parse_input_nom(i: &str) -> IResult<&str, ParsedInput<'_>> {
    alt((
        map(nom_u16, ParsedInput::Signal),
        map(alpha1, ParsedInput::Wire),
    ))
    .parse(i)
}

fn parse_gate_nom(i: &str) -> IResult<&str, ParsedGate<'_>> {
    let (i, gate_expr) = alt((
        map(
            separated_pair(parse_input_nom, tag(" AND "), parse_input_nom),
            |(a, b)| ParsedGate::And(a, b),
        ),
        map(
            separated_pair(parse_input_nom, tag(" OR "), parse_input_nom),
            |(a, b)| ParsedGate::Or(a, b),
        ),
        map(
            separated_pair(parse_input_nom, tag(" LSHIFT "), nom_u16),
            |(a, b)| ParsedGate::LShift(a, b),
        ),
        map(
            separated_pair(parse_input_nom, tag(" RSHIFT "), nom_u16),
            |(a, b)| ParsedGate::RShift(a, b),
        ),
        map(preceded(tag("NOT "), parse_input_nom), ParsedGate::Not),
        map(parse_input_nom, ParsedGate::Set),
    ))
    .parse(i)?;

    let (i, _) = tag(" -> ").parse(i)?;
    let (i, output_wire_name) = alpha1.parse(i)?;

    Ok((
        i,
        ParsedGate::OutputWire(output_wire_name, Box::new(gate_expr)),
    ))
}

// Helper function to get wire ID or assign a new one
fn get_wire_id(
    s: &str,
    wires_map: &mut FxHashMap<String, WireId>,
    next_wire_id: &mut WireId,
) -> WireId {
    *wires_map.entry(s.to_string()).or_insert_with(|| {
        let ret = *next_wire_id;
        *next_wire_id += 1;
        ret
    })
}

// Helper function to convert ParsedInput to Input using the wire mapping
fn convert_parsed_input_to_input<'a>(
    parsed: ParsedInput<'a>,
    wires_map: &mut FxHashMap<String, WireId>,
    next_wire_id: &mut WireId,
) -> Input {
    match parsed {
        ParsedInput::Signal(s) => Input::Signal(s),
        ParsedInput::Wire(w) => Input::Wire(get_wire_id(w, wires_map, next_wire_id)),
    }
}

type Cirtuit = (
    Vec<Gate>,
    Vec<WireId>,
    FxHashMap<String, WireId>,
    Vec<Vec<usize>>, // dependents list
);

fn parse_circuit(i: &str) -> Cirtuit {
    let (_, parsed_gates_with_output) = many1(map_res((parse_gate_nom, line_ending), |(g, _)| {
        Ok::<_, &str>(g)
    }))
    .parse(i)
    .unwrap();

    let mut wires_map = FxHashMap::default();
    let mut next_wire_id = 0;
    let mut final_gates = Vec::new();
    let mut gate_output_wires = Vec::new(); // Stores WireId for each gate

    for parsed_gate_with_output in parsed_gates_with_output {
        if let ParsedGate::OutputWire(output_wire_name, boxed_gate_expr) = parsed_gate_with_output {
            let output_wire_id = get_wire_id(output_wire_name, &mut wires_map, &mut next_wire_id);
            let gate_expr = *boxed_gate_expr;

            let gate = match gate_expr {
                ParsedGate::Set(input) => Gate::Set(convert_parsed_input_to_input(
                    input,
                    &mut wires_map,
                    &mut next_wire_id,
                )),
                ParsedGate::And(in1, in2) => Gate::And(
                    convert_parsed_input_to_input(in1, &mut wires_map, &mut next_wire_id),
                    convert_parsed_input_to_input(in2, &mut wires_map, &mut next_wire_id),
                ),
                ParsedGate::Or(in1, in2) => Gate::Or(
                    convert_parsed_input_to_input(in1, &mut wires_map, &mut next_wire_id),
                    convert_parsed_input_to_input(in2, &mut wires_map, &mut next_wire_id),
                ),
                ParsedGate::LShift(input, val) => Gate::LShift(
                    convert_parsed_input_to_input(input, &mut wires_map, &mut next_wire_id),
                    val,
                ),
                ParsedGate::RShift(input, val) => Gate::RShift(
                    convert_parsed_input_to_input(input, &mut wires_map, &mut next_wire_id),
                    val,
                ),
                ParsedGate::Not(input) => Gate::Not(convert_parsed_input_to_input(
                    input,
                    &mut wires_map,
                    &mut next_wire_id,
                )),
                _ => unreachable!(),
            };
            final_gates.push(gate);
            gate_output_wires.push(output_wire_id);
        }
    }

    let num_actual_wires = next_wire_id as usize;
    let mut dependents: Vec<Vec<usize>> = vec![vec![]; num_actual_wires]; // Initialize with correct size

    for (gate_idx, gate) in final_gates.iter().enumerate() {
        for input_wire_id in gate.input_wire_ids() {
            // This relies on num_actual_wires being sufficient for all input_wire_ids
            // and `dependents` being sized based on `next_wire_id`
            // If an `input_wire_id` is higher than `num_actual_wires` it would be a bug
            // but `get_wire_id` ensures sequential assignment from 0.
            // dependents.resize(input_wire_id as usize + 1, vec![]); // This was problematic, should be pre-sized
            dependents[input_wire_id as usize].push(gate_idx);
        }
    }

    // Ensure dependents vector has at least num_actual_wires entries
    // This is no longer strictly needed if `dependents` is pre-sized correctly, but harmless.
    if dependents.len() < num_actual_wires {
        dependents.resize(num_actual_wires, vec![]);
    }

    (final_gates, gate_output_wires, wires_map, dependents)
}

struct Evaluator {
    gates: Vec<Gate>,
    gate_output_wires: Vec<WireId>,
    signals: Vec<Option<u16>>,
    dependents: Vec<Vec<usize>>, // dependents[input_wire_id] = list of gate_indices that use this wire as input
    num_wires: usize,
}

impl Evaluator {
    fn new(
        gates: Vec<Gate>,
        gate_output_wires: Vec<WireId>,
        num_wires: usize,
        dependents: Vec<Vec<usize>>,
    ) -> Self {
        Self {
            gates,
            gate_output_wires,
            signals: vec![None; num_wires],
            dependents,
            num_wires,
        }
    }

    fn eval(&mut self, target_wire_id: WireId) -> u16 {
        let mut ready_gates_queue: VecDeque<usize> = VecDeque::new();
        // This keeps track of how many inputs each gate still needs to be resolved
        let mut gate_unresolved_inputs_count: Vec<usize> = vec![0; self.gates.len()];

        // Initial setup: For each gate, count its unresolved input wires
        // and if all inputs are already known (e.g., direct signals), add to queue
        for (gate_idx, gate) in self.gates.iter().enumerate() {
            let mut unresolved_count = 0;
            for input_wire_id in gate.input_wire_ids() {
                if self.signals[input_wire_id as usize].is_none() {
                    unresolved_count += 1;
                }
            }
            if unresolved_count == 0 {
                // All inputs are known signals or already set wires, can evaluate
                ready_gates_queue.push_back(gate_idx);
            }
            gate_unresolved_inputs_count[gate_idx] = unresolved_count;
        }

        while let Some(gate_idx) = ready_gates_queue.pop_front() {
            let gate = &self.gates[gate_idx];
            let output_wire_id = self.gate_output_wires[gate_idx];

            // If the output wire already has a signal, skip this gate (e.g., from initial setup)
            if self.signals[output_wire_id as usize].is_some() {
                continue;
            }

            // All inputs are guaranteed to be known here because of `gate_unresolved_inputs_count`
            if let Some(output_value) = gate.eval(&self.signals) {
                self.signals[output_wire_id as usize] = Some(output_value);

                // Check if target wire is evaluated
                if output_wire_id == target_wire_id {
                    break; // Target found, exit early
                }

                // Notify dependent gates
                for &dependent_gate_idx in &self.dependents[output_wire_id as usize] {
                    // Access directly
                    if gate_unresolved_inputs_count[dependent_gate_idx] > 0 {
                        gate_unresolved_inputs_count[dependent_gate_idx] -= 1;
                        if gate_unresolved_inputs_count[dependent_gate_idx] == 0 {
                            ready_gates_queue.push_back(dependent_gate_idx);
                        }
                    }
                }
            }
        }
        self.signals[target_wire_id as usize].unwrap()
    }

    fn reset(&mut self) {
        self.signals = vec![None; self.num_wires];
    }
}

#[distributed_slice(PUZZLES)]
pub fn puzzle0() -> String {
    let (gates, gate_output_wires, wires_map, dependents) = parse_circuit(INPUT);
    let mut evaluator = Evaluator::new(gates, gate_output_wires, wires_map.len(), dependents);
    let a_wire_id = wires_map["a"];

    let output = evaluator.eval(a_wire_id);
    format_result!(output)
}

#[distributed_slice(PUZZLES)]
pub fn puzzle1() -> String {
    let (gates, gate_output_wires, wires_map, dependents) = parse_circuit(INPUT);
    let mut evaluator = Evaluator::new(gates, gate_output_wires, wires_map.len(), dependents);
    let a_wire_id = wires_map["a"];
    let b_wire_id = wires_map["b"];

    let a_val = evaluator.eval(a_wire_id);

    evaluator.reset();

    // Override signal 'b' with 'a_val' from Part 1
    evaluator.signals[b_wire_id as usize] = Some(a_val);

    let output = evaluator.eval(a_wire_id);
    format_result!(output)
}
