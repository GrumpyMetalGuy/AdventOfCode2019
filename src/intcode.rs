use std::str::FromStr;
use text_io::*;

#[derive(Copy, Clone)]
pub enum ParameterMode {
    PositionMode,
    ImmediateMode,
}

pub struct IntCode {
    memory: Vec<i32>,
}

impl IntCode {
    pub fn new(program: &Vec<i32>) -> Self {
        IntCode {
            memory: program.to_vec(),
        }
    }

    pub fn _get_parameter_value(
        &self,
        memory_address: usize,
        parameter_mode: ParameterMode,
    ) -> i32 {
        match parameter_mode {
            ParameterMode::PositionMode => self.memory[self.memory[memory_address] as usize],
            ParameterMode::ImmediateMode => self.memory[memory_address as usize],
        }
    }

    pub fn _set_memory_address(&mut self, memory_address: i32, value: i32) -> () {
        self.memory[memory_address as usize] = value;
    }

    pub fn run(&mut self) -> () {
        let mut instruction_pointer: usize = 0;

        loop {
            let opcode = self.memory[instruction_pointer as usize];

            let parameter_modes = vec![opcode / 100 % 2, opcode / 1000 % 2]
                .iter()
                .map(|n| match n {
                    0 => ParameterMode::PositionMode,
                    1 => ParameterMode::ImmediateMode,
                    _ => panic!("Unknown parameter mode"),
                })
                .collect::<Vec<ParameterMode>>();

            match opcode % 100 {
                1 => {
                    self._set_memory_address(
                        self.memory[instruction_pointer + 3],
                        self._get_parameter_value(instruction_pointer + 1, parameter_modes[0])
                            + self
                                ._get_parameter_value(instruction_pointer + 2, parameter_modes[1]),
                    );

                    instruction_pointer += 4;
                }
                2 => {
                    self._set_memory_address(
                        self.memory[instruction_pointer + 3],
                        self._get_parameter_value(instruction_pointer + 1, parameter_modes[0])
                            * self
                            ._get_parameter_value(instruction_pointer + 2, parameter_modes[1]),
                    );

                    instruction_pointer += 4;
                }
                3 => {
                    let value: String = read!();

                    let input_i32 = i32::from_str(&value).unwrap();
                    let output_address = self._get_parameter_value(
                        instruction_pointer + 1,
                        ParameterMode::ImmediateMode,
                    );

                    self.memory[output_address as usize] = input_i32;

                    instruction_pointer += 2;
                }
                4 => {
                    print!(
                        "{}",
                        self._get_parameter_value(instruction_pointer + 1, parameter_modes[0])
                    );
                    instruction_pointer += 2;
                }
                5 => {
                    if self._get_parameter_value(instruction_pointer + 1, parameter_modes[0]) != 0 {
                        instruction_pointer = self
                            ._get_parameter_value(instruction_pointer + 2, parameter_modes[1])
                            as usize;
                    } else {
                        instruction_pointer += 3;
                    }
                }
                6 => {
                    if self._get_parameter_value(instruction_pointer + 1, parameter_modes[0]) == 0 {
                        instruction_pointer = self
                            ._get_parameter_value(instruction_pointer + 2, parameter_modes[1])
                            as usize;
                    } else {
                        instruction_pointer += 3;
                    }
                }
                7 => {
                    if self._get_parameter_value(instruction_pointer + 1, parameter_modes[0])
                        < self._get_parameter_value(instruction_pointer + 2, parameter_modes[1])
                    {
                        self._set_memory_address(self.memory[instruction_pointer + 3], 1);
                    } else {
                        self._set_memory_address(self.memory[instruction_pointer + 3], 0);
                    }

                    instruction_pointer += 4;
                }
                8 => {
                    if self._get_parameter_value(instruction_pointer + 1, parameter_modes[0])
                        == self._get_parameter_value(instruction_pointer + 2, parameter_modes[1])
                    {
                        self._set_memory_address(self.memory[instruction_pointer + 3], 1);
                    } else {
                        self._set_memory_address(self.memory[instruction_pointer + 3], 0);
                    }

                    instruction_pointer += 4;
                }
                99 => break,
                x => panic!(format!("Unable to execute program, found {}", x).to_string()),
            }
        }
    }

    pub fn memory(&self) -> &Vec<i32> {
        &self.memory
    }

    pub fn reset(&mut self, program: &Vec<i32>) -> () {
        self.memory = program.to_vec();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_simple_non_parameterised_run() {
        let mut intcode = IntCode::new(&vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]);
        intcode.run();
        assert_eq!(intcode.memory()[0], 3500);
    }

    #[test]
    fn test_simple_parameterised_run() {
        let mut intcode = IntCode::new(&vec![1002, 4, 3, 4, 33]);
        intcode.run();
        assert_eq!(intcode.memory()[4], 99);
    }

    #[test]
    fn test_negative_parameter() {
        let mut intcode = IntCode::new(&vec![1101, 100, -1, 4, 0]);
        intcode.run();
        assert_eq!(intcode.memory()[4], 99);
    }
}
