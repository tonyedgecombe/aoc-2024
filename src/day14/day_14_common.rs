pub struct Robot {
    pub position: (i64, i64),
    pub velocity: (i64, i64),
}

pub fn parse_data(data: &str) -> Vec<Robot> {
    data
        .lines()
        .filter(|x| !x.is_empty())
        .map(|line| parse_line(line))
        .collect()
}

fn parse_line(line: &str) -> Robot {
    let parts: Vec<&str> = line
        .split(" ")
        .collect();

    assert_eq!(parts.len(), 2);

    let position = parse_vector(parts[0]);
    let velocity = parse_vector(parts[1]);

    Robot { position, velocity }
}

fn parse_vector(data: &str) -> (i64, i64) {
    let data = &data[2..];
    let parts: Vec<&str> = data
        .split(",")
        .collect();

    assert_eq!(parts.len(), 2);

    let x = parts[0].parse().unwrap();
    let y = parts[1].parse().unwrap();

    (x, y)
}

pub fn step(robots: &mut Vec<Robot>, width: i64, height: i64) {
    robots.iter_mut().for_each(|robot| {
        robot.position.0 = (robot.position.0 + robot.velocity.0 + width) % width;
        robot.position.1 = (robot.position.1 + robot.velocity.1 + height) % height;
    });
}

pub fn safety_factor(robots: &mut Vec<Robot>, width: i64, height: i64) -> usize {
    let x_center = (width - 1) / 2;
    let y_center = (height - 1) / 2;

    let top_left = robots.iter().filter(|robot| robot.position.0 < x_center && robot.position.1 < y_center).count();
    let top_right = robots.iter().filter(|robot| robot.position.0 > x_center && robot.position.1 < y_center).count();
    let bottom_left = robots.iter().filter(|robot| robot.position.0 < x_center && robot.position.1 > y_center).count();
    let bottom_right = robots.iter().filter(|robot| robot.position.0 > x_center && robot.position.1 > y_center).count();

    top_left * top_right * bottom_left * bottom_right
}

pub const EXAMPLE_DATA: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_data() {
        let robots = parse_data(EXAMPLE_DATA);

        assert_eq!(robots.len(), 12);

        let robot = &robots[0];
        assert_eq!(robot.position, (0, 4));
        assert_eq!(robot.velocity, (3, -3));

        let robot = &robots[11];
        assert_eq!(robot.position, (9, 5));
        assert_eq!(robot.velocity, (-3, -3));
    }

    #[test]
    fn test_one_step() {
        let mut robots = parse_data(EXAMPLE_DATA);

        step(&mut robots, 11, 7);

        let robot = &robots[0];
        assert_eq!(robot.position, (3, 1));

        let robot = &robots[11];
        assert_eq!(robot.position, (6, 2));
    }

    #[test]
    fn test_two_steps() {
        let mut robots = parse_data(EXAMPLE_DATA);

        step(&mut robots, 11, 7);
        step(&mut robots, 11, 7);

        let robot = &robots[0];
        assert_eq!(robot.position, (6, 5));

        let robot = &robots[11];
        assert_eq!(robot.position, (3, 6));
    }

    #[test]
    fn test_safety_factor() {
        let mut robots = parse_data(EXAMPLE_DATA);

        (0..100).for_each(|_| {step(&mut robots, 11, 7);});

        let safety_factor = safety_factor(&mut robots, 11, 7);

        assert_eq!(safety_factor, 12);
    }
}