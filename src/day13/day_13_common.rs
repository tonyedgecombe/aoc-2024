pub struct Machine {
    pub prize: (i64, i64),
    pub button_a: (i64, i64),
    pub button_b: (i64, i64),
}

pub static EXAMPLE_DATA: &[u8] = b"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

pub fn parse_data(data: &[u8]) -> Vec<Machine> {
    let mut machines = Vec::new();

    let mut lines = data
        .split(|&b| b == b'\n')
        .filter(|line| !line.is_empty())
        .into_iter();

    while let Some(machine) = parse_machine(&mut lines) {
        machines.push(machine);
    }

    machines
}

fn parse_machine<'a, I>(mut lines: I) -> Option<Machine>
    where I : Iterator<Item = &'a [u8]>
{
    let Some(button_a) = lines.next() else {
        return None;
    };

    let Some(button_b) = lines.next() else {
        return None;
    };

    let Some(prize) = lines.next() else {
        return None;
    };

    let button_a = parse_line(button_a);
    let button_b = parse_line(button_b);
    let prize = parse_line(prize);

    let machine = Machine {
        prize,
        button_a,
        button_b,
    };

    Some(machine)
}

fn parse_line(line: &[u8]) -> (i64, i64) {
    let mut pos = line.iter().position(|&c| c == b'+' || c == b'=').unwrap() + 1;

    let x: i64 = parse_number(line, &mut pos);

    while pos < line.len() && line[pos] != b'+' && line[pos] != b'=' {
        pos += 1;
    }
    pos += 1;

    let y: i64 = parse_number(line, &mut pos);

    (x, y)
}

fn parse_number(line: &[u8], pos: &mut usize) -> i64 {
    let mut result: i64 = 0;
    while *pos < line.len() && line[*pos] >= b'0' && line[*pos] <= b'9' {
        result = result * 10 + (line[*pos] - b'0') as i64;
        *pos += 1;
    }

    result
}

fn machine_price(machine: &Machine) -> Option<i64> {
    let prize = machine.prize;
    let button_a = machine.button_a;
    let button_b = machine.button_b;

    let determinant = button_a.0 * button_b.1 - button_a.1 * button_b.0;

    let n = (prize.0 * button_b.1 - prize.1 * button_b.0) / determinant;
    let m = (button_a.0 * prize.1 - button_a.1 * prize.0) / determinant;

    if (button_a.0 * n + button_b.0 * m, button_a.1 * n + button_b.1 * m) == (prize.0, prize.1) {
        Some(n * 3 + m)
    } else {
        None
    }
}

pub fn total_price(machines: &[Machine]) -> i64 {
    machines.iter().filter_map(|machine| machine_price(machine)).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_number() {
        let line = b"+1234,";
        let result = parse_number(line, &mut 1);
        assert_eq!(result, 1234);
    }

    #[test]
    fn test_parse_line() {
        let pair = parse_line(b"Button A: X+94, Y+34");
        assert_eq!(pair, (94, 34));
    }

    #[test]
    fn test_parse_machine() {
        let data = b"Button A: X+26, Y+66\nButton B: X+67, Y+21\nPrize: X=12748, Y=12176";
        let lines = data.split(|&c| c == b'\n').into_iter();

        let Some(machine) = parse_machine(lines) else {
            panic!()
        };

        assert_eq!(machine.button_a, (26, 66));
        assert_eq!(machine.button_b, (67, 21));
        assert_eq!(machine.prize, (12748, 12176));
    }

    #[test]
    fn test_parse_data() {
        let machines = parse_data(EXAMPLE_DATA);
        assert_eq!(machines.len(), 4);

        let machine = &machines[0];

        assert_eq!(machine.button_a, (94, 34));
        assert_eq!(machine.button_b, (22, 67));
        assert_eq!(machine.prize, (8400, 5400));
    }

    #[test]
    fn test_machine_price() {
        let machine = Machine {
            prize: (8400, 5400),
            button_a: (94, 34),
            button_b: (22, 67),
        };

        let Some(solution) = machine_price(&machine) else {
            panic!();
        };

        assert_eq!(solution, 280);
    }
}