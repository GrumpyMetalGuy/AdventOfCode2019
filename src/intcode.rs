use std::str::FromStr;
use text_io::*;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ParameterMode {
    PositionMode,
    ImmediateMode,
    RelativeMode,
}

pub type RegisterSize = i64;

pub struct IntCodeInterpreter {
    memory: Vec<RegisterSize>,
    inputs: Vec<RegisterSize>,
    output: String,
    show_output: bool,
    pipe_mode: bool,
    running: bool,
    instruction_pointer: usize,
    relative_offset: RegisterSize,
    memory_size: usize,
}

impl IntCodeInterpreter {
    pub fn new() -> Self {
        IntCodeInterpreter {
            memory: Vec::new(),
            inputs: Vec::new(),
            output: String::new(),
            show_output: true,
            pipe_mode: false,
            running: false,
            instruction_pointer: 0,
            relative_offset: 0,
            memory_size: 0,
        }
    }

    pub fn _get_parameter_value(
        &self,
        address_offset: usize,
        parameter_mode: ParameterMode,
    ) -> RegisterSize {
        let target_memory = self.memory[self.instruction_pointer + address_offset];

        match parameter_mode {
            ParameterMode::PositionMode => self.memory[target_memory as usize],
            ParameterMode::ImmediateMode => self.memory[self.instruction_pointer + address_offset],
            ParameterMode::RelativeMode => {
                self.memory[(target_memory + self.relative_offset) as usize]
            }
        }
    }

    fn _read_memory(&self, target_address: usize) -> RegisterSize {
        self.memory[target_address]
    }

    pub fn _set_memory_address(
        &mut self,
        address_offset: usize,
        value: RegisterSize,
        parameter_mode: ParameterMode,
    ) -> () {
        let target_address = self.memory[(self.instruction_pointer + address_offset) as usize];

        match parameter_mode {
            ParameterMode::PositionMode => {
                self.memory[target_address as usize] = value;
            }
            ParameterMode::RelativeMode => {
                self.memory[(target_address + self.relative_offset) as usize] = value
            }
            ParameterMode::ImmediateMode => panic!("Unexpected write mode detected"),
        }
    }

    pub fn set_show_output(&mut self, show_output: bool) {
        self.show_output = show_output;
    }

    pub fn set_memory_size(&mut self, memory_size: usize) {
        self.memory_size = memory_size;
    }

    pub fn set_pipe_mode(&mut self, pipe_mode: bool) {
        self.pipe_mode = pipe_mode;
    }

    pub fn set_inputs(&mut self, inputs: &Vec<RegisterSize>) {
        self.inputs = inputs.clone();
    }

    pub fn add_input(&mut self, input: RegisterSize) {
        self.inputs.push(input);
    }

    pub fn run(&mut self) -> () {
        if self.memory.len() < self.memory_size {
            // Ensure we have a suitable amount of memory as requested by the user
            self.memory.extend(vec![0i64; self.memory_size]);
        }

        self.running = true;

        loop {
            let opcode = self.memory[self.instruction_pointer as usize];

            // Kind of gross, but prepend with a bunch of zeroes in case we need a default
            let mut opcode_string = "00000000000".to_owned() + opcode.to_string().as_str();
            opcode_string.pop();
            opcode_string.pop();

            let parameter_modes = opcode_string
                .chars()
                .rev()
                .map(|n| match n {
                    '0' => ParameterMode::PositionMode,
                    '1' => ParameterMode::ImmediateMode,
                    '2' => ParameterMode::RelativeMode,
                    _ => panic!("Unknown parameter mode"),
                })
                .collect::<Vec<ParameterMode>>();

            match opcode % 100 {
                1 => {
                    self._set_memory_address(
                        3,
                        self._get_parameter_value(1, parameter_modes[0])
                            + self._get_parameter_value(2, parameter_modes[1]),
                        parameter_modes[2],
                    );

                    self.instruction_pointer += 4;
                }
                2 => {
                    self._set_memory_address(
                        3,
                        self._get_parameter_value(1, parameter_modes[0])
                            * self._get_parameter_value(2, parameter_modes[1]),
                        parameter_modes[2],
                    );

                    self.instruction_pointer += 4;
                }
                3 => {
                    if self.inputs.is_empty() {
                        if self.pipe_mode {
                            break;
                        }

                        let value: String = read!();

                        let input = RegisterSize::from_str(&value).unwrap();

                        self._set_memory_address(1, input, parameter_modes[0]);
                    } else {
                        let buffer_input = self.inputs.remove(0);
                        self._set_memory_address(1, buffer_input, parameter_modes[0])
                    }

                    self.instruction_pointer += 2;
                }
                4 => {
                    let current_output = self._get_parameter_value(1, parameter_modes[0]);

                    if self.show_output {
                        print!("{}", current_output);
                    }

                    self.output += current_output.to_string().as_str();

                    self.instruction_pointer += 2;
                }
                5 => {
                    if self._get_parameter_value(1, parameter_modes[0]) != 0 {
                        self.instruction_pointer =
                            self._get_parameter_value(2, parameter_modes[1]) as usize;
                    } else {
                        self.instruction_pointer += 3;
                    }
                }
                6 => {
                    if self._get_parameter_value(1, parameter_modes[0]) == 0 {
                        self.instruction_pointer =
                            self._get_parameter_value(2, parameter_modes[1]) as usize;
                    } else {
                        self.instruction_pointer += 3;
                    }
                }
                7 => {
                    if self._get_parameter_value(1, parameter_modes[0])
                        < self._get_parameter_value(2, parameter_modes[1])
                    {
                        self._set_memory_address(3, 1, parameter_modes[2]);
                    } else {
                        self._set_memory_address(3, 0, parameter_modes[2]);
                    }

                    self.instruction_pointer += 4;
                }
                8 => {
                    if self._get_parameter_value(1, parameter_modes[0])
                        == self._get_parameter_value(2, parameter_modes[1])
                    {
                        self._set_memory_address(3, 1, parameter_modes[2]);
                    } else {
                        self._set_memory_address(3, 0, parameter_modes[2]);
                    }

                    self.instruction_pointer += 4;
                }
                9 => {
                    let offset = self._get_parameter_value(1, parameter_modes[0]);
                    self.relative_offset += offset;

                    self.instruction_pointer += 2;
                }
                99 => {
                    self.running = false;
                    break;
                }
                x => panic!(format!("Unable to execute program, found {}", x).to_string()),
            }
        }
    }

    pub fn output(&self) -> &String {
        &self.output
    }

    pub fn halted(&self) -> bool {
        !self.running
    }

    pub fn memory(&self) -> &Vec<RegisterSize> {
        &self.memory
    }

    pub fn reset(&mut self, program: &Vec<RegisterSize>) -> () {
        self.instruction_pointer = 0;
        self.relative_offset = 0;
        self.memory = program.to_vec();
        self.output = String::new();
        self.inputs = Vec::new();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_simple_non_parameterised_run() {
        let mut interpreter = IntCodeInterpreter::new();
        interpreter.reset(&&vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]);
        interpreter.run();
        assert_eq!(interpreter.memory()[0], 3500);
    }

    #[test]
    fn test_simple_parameterised_run() {
        let mut interpreter = IntCodeInterpreter::new();
        interpreter.reset(&vec![1002, 4, 3, 4, 33]);
        interpreter.run();
        assert_eq!(interpreter.memory()[4], 99);
    }

    #[test]
    fn test_negative_parameter() {
        let mut interpreter = IntCodeInterpreter::new();
        interpreter.reset(&vec![1101, 100, -1, 4, 0]);
        interpreter.run();
        assert_eq!(interpreter.memory()[4], 99);
    }

    #[test]
    fn test_basic_addition() {
        let mut interpreter = IntCodeInterpreter::new();
        interpreter.reset(&vec![1, 0, 0, 0, 99]);
        interpreter.run();
        assert_eq!(interpreter.memory(), &[2, 0, 0, 0, 99]);

        interpreter.reset(&vec![1, 1, 1, 4, 99, 5, 6, 0, 99]);
        interpreter.run();
        assert_eq!(interpreter.memory(), &[30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }

    #[test]
    fn test_basic_multiplication() {
        let mut interpreter = IntCodeInterpreter::new();
        interpreter.reset(&vec![2, 3, 0, 3, 99]);
        interpreter.run();
        assert_eq!(interpreter.memory(), &[2, 3, 0, 6, 99]);

        interpreter.reset(&vec![2, 4, 4, 5, 99, 0]);
        interpreter.run();
        assert_eq!(interpreter.memory(), &[2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn test_basic_input_output() {
        let mut interpreter = IntCodeInterpreter::new();
        interpreter.set_pipe_mode(true);
        interpreter.reset(&vec![3, 0, 4, 0, 99]);
        interpreter.add_input(14);
        interpreter.run();
        assert_eq!(interpreter.memory(), &[14, 0, 4, 0, 99]);
        assert_eq!(interpreter.output, "14");
    }

    #[test]
    fn test_relative_mode() {
        let mut interpreter = IntCodeInterpreter::new();
        interpreter.set_memory_size(100000);
        interpreter.reset(&vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ]);
        interpreter.run();
        assert_eq!(
            interpreter.output(),
            "1091204-1100110011001008100161011006101099"
        );
    }

    #[test]
    fn test_large_number_support() {
        let mut interpreter = IntCodeInterpreter::new();
        interpreter.set_memory_size(100000);
        let test_number: RegisterSize = 34915192;
        interpreter.reset(&vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0]);
        interpreter.run();

        assert_eq!(
            i64::from_str(interpreter.output()).unwrap(),
            test_number * test_number,
        );
    }
}
