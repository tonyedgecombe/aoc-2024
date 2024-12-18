
pub struct Machine {
    pub register_a: i64,
    pub register_b: i64,
    pub register_c: i64,
    pub instruction_pointer: usize,
    pub program: Vec<i64>,
}

impl Machine {
    pub fn combo_operand(&self, operand: i64) -> i64 {
        match operand {
            0..=3 => operand,
            4 => self.register_a,
            5 => self.register_b,
            6 => self.register_c,
            _ => panic!(),
        }
    }
}

pub fn parse_data(data: &str) -> Machine {
    let mut machine = Machine {
        register_a: 0,
        register_b: 0,
        register_c: 0,
        instruction_pointer: 0,
        program: vec![],
    };

    for line in data.lines() {
        let parts: Vec<&str> = line.split(":").collect();
        if line.starts_with("Register A") {
            machine.register_a = parts[1].trim().parse::<i64>().unwrap();
        } else if line.starts_with("Register B") {
            machine.register_b = parts[1].trim().parse::<i64>().unwrap();
        } else if line.starts_with("Register C") {
            machine.register_c = parts[1].trim().parse::<i64>().unwrap();
        } else if line.starts_with("Program") {
            machine.program = parts[1].trim().split(",").map(|inst| inst.parse::<i64>().unwrap()).collect();
        }
    }


    machine
}

pub fn run_instructions(machine: &mut Machine) -> Vec<i64> {
    let mut result = vec![];

    while machine.instruction_pointer < machine.program.len() {
        let operator = machine.program[machine.instruction_pointer];
        let operand = machine.program[machine.instruction_pointer + 1];

        match operator {
            0 => { // adv
                let numerator = machine.register_a;
                let operand = machine.combo_operand(operand);
                let denominator = 2_i32.pow(operand as u32);

                machine.register_a = numerator / (denominator as i64);
                machine.instruction_pointer += 2;
            },
            1 => { // bxl
                machine.register_b = machine.register_b ^ operand;
                machine.instruction_pointer += 2;
            },
            2 => { // bst
                machine.register_b = machine.combo_operand(operand) % 8;
                machine.instruction_pointer += 2;
            },
            3 => { // jnz
                if machine.register_a == 0 {
                    machine.instruction_pointer += 2;
                } else {
                    machine.instruction_pointer = operand as usize;
                }
            },
            4 => { // bxc
                machine.register_b = machine.register_b ^ machine.register_c;
                machine.instruction_pointer += 2;
            },
            5 => { // out
                result.push(machine.combo_operand(operand) % 8);
                machine.instruction_pointer += 2;
            },
            6 => { // bdv
                let numerator = machine.register_a;
                let operand = machine.combo_operand(operand);
                let denominator = 2_i32.pow(operand as u32);

                machine.register_b = numerator / (denominator as i64);
                machine.instruction_pointer += 2;
            },
            7 => { // cdv
                let numerator = machine.register_a;
                let operand = machine.combo_operand(operand);
                let denominator = 2_i32.pow(operand as u32);

                machine.register_c = numerator / (denominator as i64);
                machine.instruction_pointer += 2;
            },
            _ => panic!()
        }
    }

    result
}

pub fn run_with_a(machine: &mut Machine, a: i64) -> Vec<i64> {
    machine.register_a = a;
    machine.instruction_pointer = 0;

    run_instructions(machine)
}

pub const EXAMPLE_DATA: &str = "
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

pub const EXAMPLE_DATA_2: &str = "
Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0
";

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_data() {
        let machine = parse_data(EXAMPLE_DATA);

        assert_eq!(machine.register_a, 729);
        assert_eq!(machine.register_b, 0);
        assert_eq!(machine.register_c, 0);

        assert_eq!(machine.instruction_pointer, 0);

        assert_eq!(machine.program, [0,1,5,4,3,0])
    }

    #[test]
    fn test_combo_operand() {
        let machine = Machine {
            register_a: 10,
            register_b: 20,
            register_c: 30,
            instruction_pointer: 40,
            program: vec![],
        };

        assert_eq!(machine.combo_operand(0), 0);
        assert_eq!(machine.combo_operand(1), 1);
        assert_eq!(machine.combo_operand(2), 2);
        assert_eq!(machine.combo_operand(3), 3);
        assert_eq!(machine.combo_operand(4), 10);
        assert_eq!(machine.combo_operand(5), 20);
        assert_eq!(machine.combo_operand(6), 30);
    }

    mod test_instructions {
        use super::*;

        #[test]
        fn test_adv() {
            let mut machine = Machine {
                register_a: 128,
                register_b: 0,
                register_c: 0,
                instruction_pointer: 0,
                program: vec![0, 3],
            };

            run_instructions(&mut machine);

            assert_eq!(machine.register_a, 16); // 16 = 128/(2^3)
            assert_eq!(machine.instruction_pointer, 2);
        }

        #[test]
        fn test_bxl() {
            let mut machine = Machine {
                register_a: 0,
                register_b: 10,
                register_c: 0,
                instruction_pointer: 0,
                program: vec![1, 5],
            };

            run_instructions(&mut machine);

            assert_eq!(machine.register_b, 15);
            assert_eq!(machine.instruction_pointer, 2);
        }

        #[test]
        fn test_bst() {
            let mut machine = Machine{
                register_a: 65,
                register_b: 0,
                register_c: 0,
                instruction_pointer: 0,
                program: vec![2, 4],
            };

            run_instructions(&mut machine);

            assert_eq!(machine.register_b, 1);
            assert_eq!(machine.instruction_pointer, 2);
        }

        #[test]
        fn test_bxc() {
            let mut machine = Machine {
                register_a: 0,
                register_b: 10,
                register_c: 5,
                instruction_pointer: 0,
                program: vec![4, 0],
            };

            run_instructions(&mut machine);

            assert_eq!(machine.register_b, 15);
            assert_eq!(machine.instruction_pointer, 2);
        }

        #[test]
        fn test_out() {
            let mut machine = Machine {
                register_a: 65,
                register_b: 0,
                register_c: 0,
                instruction_pointer: 0,
                program: vec![5, 4],
            };

            let result = run_instructions(&mut machine);

            assert_eq!(result, [1]);
            assert_eq!(machine.instruction_pointer, 2);
        }

        #[test]
        fn test_bdv() {
            let mut machine = Machine {
                register_a: 128,
                register_b: 0,
                register_c: 0,
                instruction_pointer: 0,
                program: vec![6, 3],
            };

            run_instructions(&mut machine);

            assert_eq!(machine.register_b, 16); // 16 = 128/(2^3)
            assert_eq!(machine.instruction_pointer, 2);
        }

        #[test]
        fn test_cdv() {
            let mut machine = Machine {
                register_a: 128,
                register_b: 0,
                register_c: 0,
                instruction_pointer: 0,
                program: vec![7, 3],
            };

            run_instructions(&mut machine);

            assert_eq!(machine.register_c, 16); // 16 = 128/(2^3)
            assert_eq!(machine.instruction_pointer, 2);
        }
    }

    #[test]
    fn test_example_data() {
        let mut machine = parse_data(EXAMPLE_DATA);

        let result = run_instructions(&mut machine);

        assert_eq!(result, [4,6,3,5,6,3,5,2,1,0]);
    }

    #[test]
    fn test_example_data_2() {
        let mut machine = parse_data(EXAMPLE_DATA_2);

        let result = run_with_a(&mut machine, 117440);

        assert_eq!(result, machine.program);
    }

}