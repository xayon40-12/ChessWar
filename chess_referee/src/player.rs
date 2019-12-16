use std::process::{Command,Child,ChildStdin,ChildStdout};
use std::io::{Write,Read};
use crate::board::{Board,Movement};

pub struct Player {
    name: String,
    colour: char,
    rock: u8,
    prog: Child,
}

impl Player {
    pub fn new(name: &str, colour: char) -> Player {
        let prog = Command::new(name).spawn().expect(&format!("Cannot start: {}.", name));
        let name = name.to_string();
        Player { name, colour, prog, rock: 1}
    }

    pub fn next_move(&mut self, board: &Board) -> Movement {
        let mut pin = self.prog.stdin.as_mut().expect(&format!("Cannot communicate with: {}.", &self.name));
        write!(pin, "{} {}\n{}", self.colour, self.rock, board.to_contiguous()).expect(&format!("Cannot communicate with: {}.", &self.name));

        let mut pout = self.prog.stdout.as_mut().expect(&format!("Cannot communicate with: {}.", &self.name));
        let mut coord = [0u8; 4];
        pout.read_exact(&mut coord);
        Movement::from(coord)
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}

