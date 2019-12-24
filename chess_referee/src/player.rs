use std::process::{Command,Stdio,Child};
use std::io::{Write,Read};
use crate::{Board,Movement};

pub struct Player {
    name: String,
    colour: char,
    rock: u8,
    prog: Child,
}

impl Player {
    pub fn new(name: &str, colour: char) -> Player {
        let prog = Command::new(name).stdin(Stdio::piped()).stdout(Stdio::piped()).spawn().expect(&format!("Cannot start: {}.", name));
        let name = name.to_string();
        Player { name, colour, prog, rock: 1}
    }

    pub fn next_move(&mut self, board: &Board) -> Movement {
        let pin = self.prog.stdin.as_mut().expect(&format!("Cannot communicate with: {}.", &self.name));
        write!(pin, "{} {}\n{}", self.colour, self.rock, board.to_contiguous()).expect(&format!("Cannot communicate with: {}.", &self.name));

        let pout = self.prog.stdout.as_mut().expect(&format!("Cannot communicate with: {}.", &self.name));
        let mut coord = [0u8; 4];
        pout.read_exact(&mut coord).expect(&format!("Could not read player {} input", &self.name));
        Movement::from(coord)
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn is_white(&self) -> bool {
        self.colour == 'b'
    }
}

