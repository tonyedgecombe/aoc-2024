use std::collections::HashMap;
use itertools::Itertools;

pub fn parse_data(data: &str) -> Machine {
    let mut wires = HashMap::new();
    let mut gates = Vec::new();

    for line in data.lines().filter(|line| !line.is_empty()) {
        if line.contains("->") {
            // Parse gate
            let parts: Vec<&str> = line.split("->").collect();
            assert_eq!(parts.len(), 2);

            let output = parts[1].trim();
            wires.insert(output, Wire { state: None});

            let parts: Vec<&str> = parts[0].trim().split(" ").collect();
            assert_eq!(parts.len(), 3);
            let input_1 = parts[0];
            let input_2 = parts[2];

            let gate = match parts[1] {
                "AND" => Gate {
                    input_1,
                    input_2,
                    output,
                    gate_type: GateType::And,
                },
                "OR" => Gate {
                    input_1,
                    input_2,
                    output,
                    gate_type: GateType::Or,
                },
                "XOR" => Gate {
                    input_1,
                    input_2,
                    output,
                    gate_type: GateType::Xor,
                },
                _ => panic!(),
            };


            gates.push(gate);

        } else {
            // Parse wire
            let parts: Vec<&str> = line.split(":").collect();
            assert_eq!(parts.len(), 2);
            let name = parts[0].trim();
            let state: bool = if parts[1].trim() == "1" {true} else {false};
            let wire = Wire { state: Some(state) };
            wires.insert(name, wire);
        }
    }

    Machine {
        wires,
        gates,
    }
}

#[derive(PartialEq, Debug)]
pub enum GateType {
    And,
    Or ,
    Xor,
}

#[derive(PartialEq, Debug)]
pub struct Gate<'a> {
    pub input_1: &'a str,
    pub input_2: &'a str,
    pub output: &'a str,
    pub gate_type: GateType,
}

pub struct Wire {
    state: Option<bool>,
}

pub struct Machine<'a> {
    pub wires: HashMap<&'a str, Wire>,
    pub gates: Vec<Gate<'a>>,
}

impl Machine<'_> {
    pub fn evaluate_till_completion(&mut self) -> i64 {
        while !self.z_complete() {
            self.evaluate_gates();
        }

        let zeds: Vec<&&str> = self.wires
            .keys()
            .filter(|key| key.starts_with("z"))
            .sorted()
            .rev()
            .collect();

        let mut result = 0;
        for zed in zeds {
            result <<= 1;
            if self.wires.get(zed).unwrap().state.unwrap() {
                result |= 1;
            }
        }

        result
    }

    fn evaluate_gates(&mut self) {
        for gate in self.gates.iter() {
            let wire_1 = self.wires.get(&gate.input_1).unwrap();
            let wire_2 = self.wires.get(&gate.input_2).unwrap();
            let output = gate.output;

            match (wire_1.state, wire_2.state) {
                (Some(w1), Some(w2)) => {
                    let new_value = match gate.gate_type {
                        GateType::And => w1 && w2,
                        GateType::Or => w1 || w2,
                        GateType::Xor => w1 ^ w2,
                    };

                    self.wires.insert(output, Wire { state: Some(new_value)});

                }
                _ => (),
            }
        }
    }

    fn z_complete(&self) -> bool {
        self.wires
            .iter()
            .filter(|(key, _)| key.starts_with("z"))
            .all(|(_, value)| value.state.is_some())
    }
}


const EXAMPLE_DATA: &str = "
x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02
";

pub const LARGE_EXAMPLE: &str = "
x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_data() {
        let machine = parse_data(EXAMPLE_DATA);

        assert_eq!(machine.wires.len(), 9);
        assert_eq!(machine.gates.len(), 3);

        assert_eq!(machine.gates[0], Gate {input_1: "x00", input_2: "y00", output: "z00", gate_type: GateType::And });
        assert!(machine.wires.contains_key("z00"));
    }

    #[test]
    fn test_parse_large_example() {
        let machine = parse_data(LARGE_EXAMPLE);

        assert!(machine.wires.contains_key("x00"));
        assert!(machine.wires.contains_key("x03"));
        assert!(machine.wires.contains_key("fst"));
    }

    #[test]
    fn test_evaluate_gates_and() {
        let mut machine = parse_data(EXAMPLE_DATA);
        machine.evaluate_gates();

        let wire = machine.wires.get("z00").unwrap();
        let Some(state) = wire.state else {
            panic!();
        };

        assert!(!state);
    }

    #[test]
    fn test_evaluate_gates_or() {
        let mut machine = parse_data(EXAMPLE_DATA);
        machine.evaluate_gates();

        let wire = machine.wires.get("z02").unwrap();
        let Some(state) = wire.state else {
            panic!();
        };

        assert!(state);
    }

    #[test]
    fn test_evaluate_gates_xor() {
        let mut machine = parse_data(EXAMPLE_DATA);
        machine.evaluate_gates();

        let wire = machine.wires.get("z01").unwrap();
        let Some(state) = wire.state else {
            panic!();
        };

        assert!(!state);
    }

    #[test]
    fn test_z_complete() {
        let mut machine = parse_data(EXAMPLE_DATA);
        assert!(!machine.z_complete());

        machine.evaluate_gates();
        assert!(machine.z_complete());
    }

    #[test]
    fn test_evaluate_to_completion() {
        let mut machine = parse_data(EXAMPLE_DATA);
        let result = machine.evaluate_till_completion();

        assert_eq!(result, 4);
    }

    #[test]
    fn test_evaluate_to_completion_large_example() {
        let mut machine = parse_data(LARGE_EXAMPLE);
        let result = machine.evaluate_till_completion();

        assert_eq!(result, 2024);
    }
}