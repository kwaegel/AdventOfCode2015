
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;

// Register constant table
#[derive(Copy,Clone,Debug)]
enum Registers {
    RegA = 0,
    RegB = 1,
}

#[derive(Copy,Clone,Debug)]
enum Command {
    HLF, /* hlf r sets register r to half its current value, then continues with the next instruction. */
    TPL, /* tpl r sets register r to triple its current value, then continues with the next instruction. */
    INC, // inc r increments register r, adding 1 to it, then continues with the next instruction.
    JMP, /* jmp offset is a jump; it continues with the instruction offset away relative to itself. */
    JIE, // jie r, offset is like jmp, but only jumps if register r is even ("jump if even").
    JIO, // jio r, offset is like jmp, but only jumps if register r is 1 ("jump if one", not odd).
}
impl Command {
    fn parse(text: &str) -> Command {
        match text {
            "hlf" => Command::HLF,
            "tpl" => Command::TPL,
            "inc" => Command::INC,
            "jmp" => Command::JMP,
            "jie" => Command::JIE,
            "jio" => Command::JIO,
            _ => panic!("Unknown command {}", text),
        }
    }
}

fn parse_arg_value(text: &str) -> i32 {
    match text {
        "a" | "a," => Registers::RegA as i32,
        "b" | "b," => Registers::RegB as i32,
        _ => {
            match text.parse::<i32>() {
                Ok(num) => num,
                Err(_) => panic!("Unknown argument string {}", text),
            }
        }
    }
}

#[derive(Copy,Clone,Debug)]
struct Instruction {
    command: Command,
    arg_1: i32,
    arg_2: i32,
}



fn read_program_from_file(filename: &str) -> Vec<Instruction> {
    let path = Path::new(filename);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, Error::description(&why)),
        Ok(file) => file,
    };

    let reader = BufReader::new(file);
    let lines = reader.lines();

    let mut program: Vec<Instruction> = Vec::new();
    for line in lines {
        let string = line.unwrap();
        let tokens: Vec<&str> = string.split_whitespace().collect();
        let command = Command::parse(tokens[0]);
        let arg_1 = parse_arg_value(tokens[1]);
        let arg_2 = if tokens.len() > 2 {
            parse_arg_value(tokens[2])
        } else {
            0
        };

        program.push(Instruction {
            command: command,
            arg_1: arg_1,
            arg_2: arg_2,
        });
    }

    program
}

#[derive(Copy,Clone,Debug)]
struct ProgramState {
    registers: [u32; 2],
    pc: i32, // Program counter
}

fn is_even(num: i32) -> bool {
    num % 2 == 0
}

fn process_instruction(mut state: ProgramState, instruction: Instruction) -> ProgramState {
    // hlf r sets register r to half its current value, then continues with the next instruction.
    // tpl r sets register r to triple its current value, then continues with the next instruction.
    // inc r increments register r, adding 1 to it, then continues with the next instruction.
    // jmp offset is a jump; it continues with the instruction offset away relative to itself.
    // jie r, offset is like jmp, but only jumps if register r is even ("jump if even").
    // jio r, offset is like jmp, but only jumps if register r is 1 ("jump if one", not odd).
    let Instruction { command, arg_1, arg_2 } = instruction;
    match command {
        Command::HLF => {
            state.registers[arg_1 as usize] = state.registers[arg_1 as usize] / 2;
            state.pc += 1;
        }
        Command::TPL => {
            state.registers[arg_1 as usize] = state.registers[arg_1 as usize] * 3;
            state.pc += 1;
        }
        Command::INC => {
            state.registers[arg_1 as usize] += 1;
            state.pc += 1;
        }
        Command::JMP => {
            state.pc += arg_1;
        }
        Command::JIE => {
            let reg_value = state.registers[arg_1 as usize] as i32;
            if is_even(reg_value) {
                state.pc += arg_2;
            } else {
                state.pc += 1;
            }
        }
        Command::JIO => {
            let reg_value = state.registers[arg_1 as usize];
            if reg_value == 1 {
                state.pc += arg_2;
            } else {
                state.pc += 1;
            }
        }
    }
    state
}

fn main() {
    let program = read_program_from_file("day23.txt");

    let mut state = ProgramState {
        registers: [0; 2],
        pc: 0,
    };

    // Loop until program counter indexes out of the program.
    while state.pc >= 0 && state.pc < program.len() as i32 {
        let instruction = program[state.pc as usize];
        state = process_instruction(state, instruction);
    }

    // Check value of register B
    let reg_b_val = state.registers[Registers::RegB as usize];
    println!("Part 1: final state is {:?}", state);
    assert_eq!(reg_b_val, 307);

    // **********
    // Now try with register A starting with value 1.
    state = ProgramState {
        registers: [1, 0],
        pc: 0,
    };

    // Loop until program counter indexes out of the program.
    while state.pc >= 0 && state.pc < program.len() as i32 {
        let instruction = program[state.pc as usize];
        state = process_instruction(state, instruction);
    }

    // Check value of register B
    let reg_b_val = state.registers[Registers::RegB as usize];
    println!("Part 2: final state is {:?}", state);
    assert_eq!(reg_b_val, 160);
}
