use AdventOfCode2019::intcode::IntCodeInterpreter;

fn main() {
    let program_input = vec![
        1, 0, 0, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 9, 1, 19, 1, 19, 5, 23, 1, 23, 5, 27, 2,
        27, 10, 31, 1, 31, 9, 35, 1, 35, 5, 39, 1, 6, 39, 43, 2, 9, 43, 47, 1, 5, 47, 51, 2, 6, 51,
        55, 1, 5, 55, 59, 2, 10, 59, 63, 1, 63, 6, 67, 2, 67, 6, 71, 2, 10, 71, 75, 1, 6, 75, 79,
        2, 79, 9, 83, 1, 83, 5, 87, 1, 87, 9, 91, 1, 91, 9, 95, 1, 10, 95, 99, 1, 99, 13, 103, 2,
        6, 103, 107, 1, 107, 5, 111, 1, 6, 111, 115, 1, 9, 115, 119, 1, 119, 9, 123, 2, 123, 10,
        127, 1, 6, 127, 131, 2, 131, 13, 135, 1, 13, 135, 139, 1, 9, 139, 143, 1, 9, 143, 147, 1,
        147, 13, 151, 1, 151, 9, 155, 1, 155, 13, 159, 1, 6, 159, 163, 1, 13, 163, 167, 1, 2, 167,
        171, 1, 171, 13, 0, 99, 2, 0, 14, 0,
    ];

    let mut current_program = program_input.clone();
    current_program[1] = 12;
    current_program[2] = 2;

    let mut interpreter = IntCodeInterpreter::new();
    interpreter.reset(&current_program);
    interpreter.run();

    println!("{}", interpreter.memory()[0]);

    for noun in 0..=99 {
        for verb in 0..=99 {
            current_program = program_input.clone();
            current_program[1] = noun;
            current_program[2] = verb;

            interpreter.reset(&current_program);
            interpreter.run();

            if interpreter.memory()[0] == 19690720 {
                println!("{:?}", 100 * noun + verb);
            }
        }
    }
}
