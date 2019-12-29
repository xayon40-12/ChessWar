use std::process::{Command,Stdio,Child};
use std::io::{Write,BufReader,BufRead};
use crate::Movement;

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
        Player { name, colour, prog, rock: 0} // TODO put rock to 1 and handle rock
    }

    pub fn next_move(&mut self, contiguous_board: String) -> Movement {
        let pin = self.prog.stdin.as_mut().expect(&format!("Cannot communicate with: {}.", &self.name));
        write!(pin, "{} {}\n{}\n", self.colour, self.rock, contiguous_board).expect(&format!("Cannot communicate with: {}.", &self.name));

        let mut pout = BufReader::new(self.prog.stdout.as_mut().expect(&format!("Cannot communicate with: {}.", &self.name)));
        let mut line = String::new();
        pout.read_line(&mut line).expect(&format!("Could not read player {} input", &self.name));
        Movement::from(line)
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn is_white(&self) -> bool {
        self.colour == 'b'
    }

    pub fn kill(&mut self) {
        self.prog.kill().expect(&format!("Could not end {} properly.", &self.name));
    }
}

