pub const VERSION: &str = "0.1.0";
const MAX_SIZE: usize = 30000;
pub const UPDATE_DATE: &str = "17/12/2024";
use shittyinput::get_string;
use std::io::{stdout, Write};



pub fn run_str(string: &str) {
    let mut array: [u8; MAX_SIZE] = [0; MAX_SIZE];
    let mut pointer = Pointer::new();
    let instructions = parse(string);
    run_actions(&instructions, &mut array, &mut pointer);
}

pub fn interpret_ui() {
    let mut array: [u8; MAX_SIZE] = [0; MAX_SIZE];
    let mut pointer = Pointer::new();
    println!("Brainfck interpreter by leo. Version {VERSION} {UPDATE_DATE}");
    println!(r#"Type "exit" to exit"#);
    loop {
        print!(">>> ");
        let _ = stdout().flush();
        let input = get_string();
        let input = input.trim();
        if input == "exit" {
            break;
        }
        let instructions = parse(input);
        run_actions(&instructions, &mut array, &mut pointer);
    }
}

fn parse(string: &str) -> Vec<Actions> {
    let mut arr: Vec<Actions> = vec![];
    for index in 0..string.len() {
        match string.as_bytes()[index] {
            b'+' => arr.push(Actions::ADD),
            b'-' => arr.push(Actions::SUBTRACT),
            b'<' => arr.push(Actions::LEFT),
            b'>' => arr.push(Actions::RIGHT),
            b'[' => arr.push(Actions::LOOPOPEN),
            b']' => arr.push(Actions::LOOPCLOSE),
            b'.' => arr.push(Actions::OUTPUT),
            b',' => arr.push(Actions::INPUT),
            _ => arr.push(Actions::NONE),
        }
    }
    return arr;
}

fn run_actions(instructions: &Vec<Actions>, array: &mut [u8; MAX_SIZE], pointer: &mut Pointer) {
    for index in 0..instructions.len() {
        match instructions[index] {
            Actions::ADD => {
                if array[pointer.get()] != 255 {
                    array[pointer.get()] += 1;
                }
            },
            Actions::SUBTRACT => {
                if array[pointer.get()] != 0 {
                    array[pointer.get()] -= 1;
                }
            },
            Actions::LEFT => {
                pointer.left();
            },
            Actions::RIGHT => {
                pointer.right();
            },
            Actions::LOOPOPEN => {
                let mut temp_index = index+1;
                let mut new_instructions: Vec<Actions> = vec![];
                while instructions[temp_index] != Actions::LOOPCLOSE {
                    new_instructions.push(instructions[temp_index]);
                    temp_index += 1;
                }
                new_instructions.push(instructions[temp_index]);
                while array[pointer.get()] != 1 {
                    run_actions(&new_instructions, array, pointer);
                }
            },
            Actions::OUTPUT => {
                print!("{}", (array[pointer.get()]%255) as u8 as char);
            },
            Actions::INPUT => {
                stdout().flush().unwrap_or(()); array[pointer.get()] = get_string().as_bytes()[0];
            },
            Actions::NONE | Actions::LOOPCLOSE => {},
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Actions {
    ADD,
    SUBTRACT ,
    LEFT ,
    RIGHT,
    LOOPOPEN,
    LOOPCLOSE,
    OUTPUT,
    INPUT,
    NONE,
}

struct Pointer {
    index: usize,
}
impl Pointer {
    fn left(&mut self) {
        if self.index == 0 {
            self.index = MAX_SIZE-1;
            return
        } 
        self.index -= 1;
    }
    fn right(&mut self) {
        self.index = (self.index + 1)%MAX_SIZE;
    }
    fn new() -> Self {
        return Pointer {index: 0};
    }
    fn get(&self) -> usize {
        return self.index;
    }
}
