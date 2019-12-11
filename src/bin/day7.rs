use permutohedron::Heap;
use std::str::FromStr;
use AdventOfCode2019::intcode::{IntCodeInterpreter, RegisterSize};

fn run_part_one(program: &Vec<RegisterSize>, controls: &mut Vec<RegisterSize>) -> RegisterSize {
    let mut results = Vec::<RegisterSize>::new();
    let permutor = Heap::new(controls);

    let mut interpreter = IntCodeInterpreter::new();
    interpreter.set_show_output(false);

    for phases in permutor {
        let mut input_signal = 0;

        for phase in phases {
            let current_program = program.clone();
            interpreter.reset(&current_program);
            interpreter.set_inputs(&vec![phase, input_signal]);

            interpreter.run();
            input_signal = RegisterSize::from_str(interpreter.output()).unwrap();
        }

        results.push(input_signal);
    }

    *results.iter().max().unwrap()
}

fn run_part_two(program: &Vec<RegisterSize>, controls: &mut Vec<RegisterSize>) -> RegisterSize {
    let mut results = Vec::<RegisterSize>::new();
    let permutor = Heap::new(controls);

    let mut interpreter_a = IntCodeInterpreter::new();
    interpreter_a.set_show_output(false);
    interpreter_a.set_pipe_mode(true);
    let mut interpreter_b = IntCodeInterpreter::new();
    interpreter_b.set_show_output(false);
    interpreter_b.set_pipe_mode(true);
    let mut interpreter_c = IntCodeInterpreter::new();
    interpreter_c.set_show_output(false);
    interpreter_c.set_pipe_mode(true);
    let mut interpreter_d = IntCodeInterpreter::new();
    interpreter_d.set_show_output(false);
    interpreter_d.set_pipe_mode(true);
    let mut interpreter_e = IntCodeInterpreter::new();
    interpreter_e.set_show_output(false);
    interpreter_e.set_pipe_mode(true);

    for phases in permutor {
        let mut input_signal: RegisterSize;

        interpreter_a.reset(&program.clone());
        interpreter_a.add_input(*phases.iter().nth(0).unwrap());
        interpreter_a.add_input(0);
        interpreter_b.reset(&program.clone());
        interpreter_b.add_input(*phases.iter().nth(1).unwrap());
        interpreter_c.reset(&program.clone());
        interpreter_c.add_input(*phases.iter().nth(2).unwrap());
        interpreter_d.reset(&program.clone());
        interpreter_d.add_input(*phases.iter().nth(3).unwrap());
        interpreter_e.reset(&program.clone());
        interpreter_e.add_input(*phases.iter().nth(4).unwrap());

        loop {
            interpreter_a.run();

            input_signal = RegisterSize::from_str(interpreter_a.output()).unwrap();
            interpreter_b.add_input(input_signal);

            interpreter_b.run();
            input_signal = RegisterSize::from_str(interpreter_b.output()).unwrap();
            interpreter_c.add_input(input_signal);

            interpreter_c.run();
            input_signal = RegisterSize::from_str(interpreter_c.output()).unwrap();
            interpreter_d.add_input(input_signal);

            interpreter_d.run();
            input_signal = RegisterSize::from_str(interpreter_d.output()).unwrap();
            interpreter_e.add_input(input_signal);

            interpreter_e.run();
            input_signal = RegisterSize::from_str(interpreter_e.output()).unwrap();

            if interpreter_e.halted() {
                results.push(input_signal);
                break;
            } else {
                interpreter_a.add_input(input_signal);
            }
        }
    }

    *results.iter().max().unwrap()
}

fn main() {
    let program = vec![
        3, 8, 1001, 8, 10, 8, 105, 1, 0, 0, 21, 34, 51, 76, 101, 126, 207, 288, 369, 450, 99999, 3,
        9, 102, 4, 9, 9, 1001, 9, 2, 9, 4, 9, 99, 3, 9, 1001, 9, 2, 9, 1002, 9, 3, 9, 101, 3, 9, 9,
        4, 9, 99, 3, 9, 102, 5, 9, 9, 1001, 9, 2, 9, 102, 2, 9, 9, 101, 3, 9, 9, 1002, 9, 2, 9, 4,
        9, 99, 3, 9, 101, 5, 9, 9, 102, 5, 9, 9, 1001, 9, 2, 9, 102, 3, 9, 9, 1001, 9, 3, 9, 4, 9,
        99, 3, 9, 101, 2, 9, 9, 1002, 9, 5, 9, 1001, 9, 5, 9, 1002, 9, 4, 9, 101, 5, 9, 9, 4, 9,
        99, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9,
        102, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 1001, 9, 2,
        9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9,
        99, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9,
        1002, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 1001, 9, 2,
        9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 99,
        3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 1001,
        9, 2, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4,
        9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 99, 3,
        9, 1001, 9, 2, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1001, 9,
        1, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9,
        3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 99, 3, 9,
        1001, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9,
        9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3,
        9, 102, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 99,
    ];

    // Part 1
    println!(
        "Max result: {}",
        run_part_one(&program.clone(), &mut vec![0, 1, 2, 3, 4])
    );

    // Part 2
    println!(
        "Max result: {}",
        run_part_two(&program.clone(), &mut vec![5, 6, 7, 8, 9])
    );
}
