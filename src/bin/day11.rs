use std::collections::HashMap;
use AdventOfCode2019::intcode::IntCodeInterpreter;

type Point = (i64, i64);

enum Direction {
    North,
    South,
    East,
    West,
}

struct EmergencyHullPaintingRobot {
    location: Point,
    direction: Direction,
}

impl EmergencyHullPaintingRobot {
    fn new() -> Self {
        EmergencyHullPaintingRobot {
            location: (0, 0),
            direction: Direction::North,
        }
    }

    fn rotate_left(&mut self) {
        self.direction = match &self.direction {
            Direction::North => Direction::West,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
            Direction::East => Direction::North,
        };
    }

    fn rotate_right(&mut self) {
        self.direction = match &self.direction {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
            Direction::East => Direction::South,
        };
    }

    fn advance(&mut self) {
        match &self.direction {
            Direction::North => self.location.1 -= 1,
            Direction::South => self.location.1 += 1,
            Direction::West => self.location.0 -= 1,
            Direction::East => self.location.0 += 1,
        };
    }
}

fn paint_hull(
    starting_colour: i64,
    robot: &mut EmergencyHullPaintingRobot,
    hull: &mut HashMap<Point, i64>,
    interpreter: &mut IntCodeInterpreter,
) {
    interpreter.add_input(starting_colour);

    loop {
        interpreter.run();

        if interpreter.halted() {
            break;
        }

        let colour = match interpreter.output().chars().nth(0).unwrap() {
            '0' => 0,
            '1' => 1,
            _ => panic!("Unexpected colour found"),
        };

        match interpreter.output().chars().nth(1).unwrap() {
            '0' => {
                robot.rotate_left();
            }
            '1' => {
                robot.rotate_right();
            }
            _ => panic!("Unexpected colour found"),
        }

        interpreter.clear_output();

        hull.insert(robot.location, colour);

        robot.advance();

        if hull.contains_key(&robot.location) {
            interpreter.add_input(*hull.get(&robot.location).unwrap());
        } else {
            interpreter.add_input(0);
        }
    }
}

fn main() {
    let program = vec![3,8,1005,8,319,1106,0,11,0,0,0,104,1,104,0,3,8,1002,8,-1,10,101,1,10,10,4,10,108,0,8,10,4,10,1002,8,1,28,3,8,1002,8,-1,10,1001,10,1,10,4,10,1008,8,1,10,4,10,102,1,8,51,2,1008,18,10,3,8,1002,8,-1,10,1001,10,1,10,4,10,1008,8,1,10,4,10,101,0,8,77,1,1006,8,10,1006,0,88,3,8,1002,8,-1,10,1001,10,1,10,4,10,1008,8,1,10,4,10,1002,8,1,106,1006,0,47,2,5,0,10,3,8,102,-1,8,10,101,1,10,10,4,10,1008,8,0,10,4,10,101,0,8,135,2,105,3,10,2,1101,6,10,3,8,102,-1,8,10,101,1,10,10,4,10,1008,8,0,10,4,10,1002,8,1,165,3,8,102,-1,8,10,101,1,10,10,4,10,108,0,8,10,4,10,1002,8,1,186,1,1009,11,10,1,9,3,10,2,1003,10,10,1,107,11,10,3,8,1002,8,-1,10,101,1,10,10,4,10,1008,8,1,10,4,10,1002,8,1,225,1006,0,25,1,1009,14,10,1,1008,3,10,3,8,102,-1,8,10,101,1,10,10,4,10,108,1,8,10,4,10,1002,8,1,257,1,1006,2,10,3,8,1002,8,-1,10,1001,10,1,10,4,10,1008,8,0,10,4,10,101,0,8,284,2,1004,7,10,1006,0,41,2,1106,17,10,1,104,3,10,101,1,9,9,1007,9,919,10,1005,10,15,99,109,641,104,0,104,1,21101,0,937108545948,1,21102,1,336,0,1105,1,440,21102,1,386577203612,1,21102,347,1,0,1105,1,440,3,10,104,0,104,1,3,10,104,0,104,0,3,10,104,0,104,1,3,10,104,0,104,1,3,10,104,0,104,0,3,10,104,0,104,1,21102,1,21478178819,1,21102,1,394,0,1106,0,440,21102,21477985447,1,1,21101,405,0,0,1105,1,440,3,10,104,0,104,0,3,10,104,0,104,0,21101,984458351460,0,1,21101,428,0,0,1106,0,440,21101,709048034148,0,1,21102,439,1,0,1106,0,440,99,109,2,21201,-1,0,1,21101,0,40,2,21101,471,0,3,21102,461,1,0,1105,1,504,109,-2,2106,0,0,0,1,0,0,1,109,2,3,10,204,-1,1001,466,467,482,4,0,1001,466,1,466,108,4,466,10,1006,10,498,1101,0,0,466,109,-2,2105,1,0,0,109,4,2101,0,-1,503,1207,-3,0,10,1006,10,521,21101,0,0,-3,22102,1,-3,1,21201,-2,0,2,21102,1,1,3,21102,540,1,0,1106,0,545,109,-4,2105,1,0,109,5,1207,-3,1,10,1006,10,568,2207,-4,-2,10,1006,10,568,22101,0,-4,-4,1105,1,636,21201,-4,0,1,21201,-3,-1,2,21202,-2,2,3,21102,587,1,0,1106,0,545,21202,1,1,-4,21102,1,1,-1,2207,-4,-2,10,1006,10,606,21101,0,0,-1,22202,-2,-1,-2,2107,0,-3,10,1006,10,628,22101,0,-1,1,21101,628,0,0,105,1,503,21202,-2,-1,-2,22201,-4,-2,-4,109,-5,2105,1,0,];

    let mut hull: HashMap<Point, i64> = HashMap::new();

    let mut interpreter = IntCodeInterpreter::new();

    interpreter.set_memory_size(10000);
    interpreter.reset(&program);
    interpreter.set_pipe_mode(true);
    interpreter.set_show_output(false);

    let mut robot = EmergencyHullPaintingRobot::new();

    paint_hull(0, &mut robot, &mut hull, &mut interpreter);

    println!("{}", hull.len());

    // Part 2
    interpreter.reset(&program);
    hull.clear();
    paint_hull(1, &mut robot, &mut hull, &mut interpreter);

    hull = hull
        .into_iter()
        .filter(|&(_location, colour)| colour == 1)
        .collect();

    let min_x = hull
        .iter()
        .map(|(location, _colour)| location.0)
        .min()
        .unwrap();
    let min_y = hull
        .iter()
        .map(|(location, _colour)| location.1)
        .min()
        .unwrap();

    hull = hull
        .into_iter()
        .map(|(location, colour)| ((location.0 - min_x, location.1 - min_y), colour))
        .collect();

    let max_x = hull
        .iter()
        .map(|(location, _colour)| location.0)
        .max()
        .unwrap();
    let max_y = hull
        .iter()
        .map(|(location, _colour)| location.1)
        .max()
        .unwrap();

    for y in 0..=max_y {
        for x in 0..=max_x {
            if hull.contains_key(&(x, y)) {
                print!("#");
            } else {
                print!(" ");
            }
        }

        println!();
    }

    println!("Ha ha, it's upside down");
}
