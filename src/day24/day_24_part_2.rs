use crate::day24::day_24_common::*;
use itertools::Itertools;

fn day_24_part_2() -> Vec<String> {
    let mut machine = parse_data(include_str!("../data/day_24_input.txt"));
    let gates: &mut Vec<Gate> = &mut machine.gates;

    let mut result = Vec::new();

    let xor = find_gate_index_from_inputs(&gates, "x00", "y00", GateType::Xor).unwrap();
    assert_eq!(gates[xor].output, "z00");

    let and = find_gate_index_from_inputs(&gates, "x00", "y00", GateType::And).unwrap();

    let mut carry = gates[and].output; // Nothing we can do to test this right now

    let mut i = 1;
    while i < 45 {
        let x = format!("x{:02}", i);
        let y = format!("y{:02}", i);
        let z = format!("z{:02}", i);

        let xor_1 = find_gate_index_from_inputs(&gates, &x, &y, GateType::Xor).unwrap();

        let and_1 = find_gate_index_from_inputs(&gates, &x, &y, GateType::And).unwrap();

        let xor_2 = match find_gate_index_from_inputs(&gates, carry, gates[xor_1].output, GateType::Xor) {
            Some(xor_2) => xor_2,
            None => {
                // Is this the carry bit or the output from xor_1?
                if find_gate_index_from_input(&gates, gates[xor_1].output, GateType::Xor).is_some() {
                    // Carry bit is wrong
                    panic!();
                } else {
                    // Output from xor_1 is wrong
                    let xor_2 = find_gate_index_from_output(gates, &z).unwrap();
                    if gates[xor_2].gate_type != GateType::Xor {
                        panic!();
                    }

                    if gates[xor_2].input_1 != carry {
                        panic!(); // Wires crossed?
                    }

                    let other_side = find_gate_index_from_output(&gates, gates[xor_2].input_2).unwrap();

                    result.push(gates[other_side].output.to_string());
                    result.push(gates[xor_1].output.to_string());

                    // Swap back
                    let tmp = gates[other_side].output;
                    gates[other_side].output = gates[xor_1].output;
                    gates[xor_1].output = tmp;

                    continue;
                }
            },
        };

        if  gates[xor_2].output != z {
            result.push(gates[xor_2].output.to_string());
            result.push(z.clone());

            let other_side = find_gate_index_from_output(gates, &z).unwrap();

            // Swap back
            let tmp = gates[other_side].output;
            gates[other_side].output = gates[xor_2].output;
            gates[xor_2].output = tmp;

            continue;
        }

        let and_2 = find_gate_index_from_inputs(&gates, gates[xor_1].output, carry, GateType::And).unwrap();

        let or_1 = find_gate_index_from_inputs(&gates, gates[and_1].output, gates[and_2].output, GateType::Or).unwrap();


        carry = gates[or_1].output;
        i += 1;
    }

    result
}

fn find_gate_index_from_inputs(gates: &Vec<Gate>, input_1: &str, input_2: &str, gate_type: GateType) -> Option<usize> {
    for i in 0..gates.len() {
        let gate = &gates[i];
        if ((gate.input_1 == input_1 && gate.input_2 == input_2) || (gate.input_1 == input_2 && gate.input_2 == input_1)) && gate.gate_type == gate_type {
            return Some(i);
        }
    }

    None
}
fn find_gate_index_from_input(gates: &Vec<Gate>, input: &str, gate_type: GateType) -> Option<usize> {
    for i in 0..gates.len() {
        let gate = &gates[i];
        if (gate.input_1 == input || gate.input_2 == input) && gate.gate_type == gate_type {
            return Some(i);
        }
    }

    None
}

fn find_gate_index_from_output(gates: &Vec<Gate>, output: &str) -> Option<usize> {
    for i in 0..gates.len() {
        let gate = &gates[i];
        if gate.output == output {
            return Some(i);
        }
    }

    None
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day_24_part_2() {
        let result = day_24_part_2();
        let result = result.iter().sorted().join(",");

        assert_eq!(result, "ddn,kqh,nhs,nnf,wrc,z09,z20,z34");
    }
}