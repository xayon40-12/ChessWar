use crate::{Movement,Player};
use colored::*;
use std::env;
use crate::chess;

pub struct Board {
    mat: [[char; 8]; 8],
    last_movement: Movement,
    p: [Player; 2],
    i: usize
}


impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let str: String = self.mat.iter().flat_map(|l| l.iter().chain(['\n'].iter())).collect();
        write!(f, "{}", str)
    }
}

impl Board {
    pub fn initialise_players() -> [Player; 2] {
        let args: Vec<String> = env::args().collect();
        if args.len() < 3 { panic!("Too few arguments!"); }

        //first player white 'b' and second black 'n'
        [Player::new(&args[1],'b'), Player::new(&args[2],'n')]
    }

    pub fn initialise_board() -> Board {
        Board::from_str("tcfrdfctpppppppp                                PPPPPPPPTCFDRFCT")
    }

    pub fn from_str(str: &str) -> Board {
        let chars = &str.chars().collect::<Vec<_>>()[..];
        if str.len() != 64 { panic!("Wrong string length in Board::from_str().") }

        let mut mat = [[' '; 8]; 8];
        for j in 0usize..8 {
            for i in 0usize..8 {
                mat[j][i] = chars[j*8+i];
            } 
        }

        let p = Board::initialise_players();

        Board { mat, last_movement: Movement { before: (0,0), after: (0,0) }, p, i: 0 }
    }
   
    // try to apply the movement and return true if possible, false if wrong
    pub fn apply_movement(&mut self, mov: Movement, is_white: bool) -> bool {
        match chess::check_player_movement(&self.mat, &mov, is_white) {
            Ok(_) => {
                self.mat[mov.after.1][mov.after.0] = self.mat[mov.before.1][mov.before.0];
                self.mat[mov.before.1][mov.before.0] = ' ';
                self.last_movement = mov;
                true
            },
            Err(e) => {
                println!("{}", e);
                self.last_movement = mov;
                false
            }
        }
    }

    pub fn turn(&mut self) {
        println!("{}'s turn:", self.p[self.i].get_name());
        let mut end = false;
        let (i,j) = (self.i,1-self.i);
        let m = self.p[i].next_move(self.to_contiguous());
        if !self.apply_movement(m, self.p[i].is_white()) {
            self.show(true, &[]);
            println!("Player {} has made a mistake!", self.p[i].get_name());
            end = true;
        }
        let targeting = chess::check(&self.mat, self.p[i].is_white());
        if targeting.len() > 0 {
            self.show(true, &targeting);
            println!("Player {} has suicided itself!", self.p[i].get_name());
            end = true;
        }
            
        if chess::checkmate(&self.mat, !self.p[i].is_white()) {
            let targeting = chess::check(&self.mat, !self.p[i].is_white());
            self.show(false, &targeting);
            end = true;
        }

        if end {
            self.p[i].kill();
            self.p[j].kill();
            std::process::exit(0);
        } else {
            self.show(false, &[]);
        }

        self.i = 1-i;
    }

    pub fn to_contiguous(&self) -> String {
       self.mat.iter().flat_map(|l| l.iter()).collect()
    }

    pub fn show(&self, mistake: bool, targeting: &[(usize,usize)]) {
        for y in 0..8 {
            for x in 0..8 {
                let c = if self.last_movement.after == (x,y) && !mistake { "green" } else { if (x+y)%2 == 0 { "black" } else { "white" } };
                let on_c = if targeting.contains(&(x,y)) { if mistake { "red" } else { "blue" } } else { 
                    if self.last_movement.before == (x,y) { "green" } else { 
                    if mistake && self.last_movement.after == (x,y) { "red" } else { if (x+y)%2 == 0 { "white" } else { "black" } } 
                }};
                let s = /*match self.mat[y][x] {
                    'p' => "\u{2659}",
                    't' => "\u{2656}",
                    'c' => "\u{2658}",
                    'f' => "\u{2657}",
                    'r' => "\u{2654}",
                    'd' => "\u{2655}",
                    'P' => "\u{265F}",
                    'T' => "\u{265C}",
                    'C' => "\u{265E}",
                    'F' => "\u{265D}",
                    'R' => "\u{265A}",
                    'D' => "\u{265B}",
                    ' ' => " ",
                    _ => panic!("Unrecognised character!")
                };*/self.mat[y][x].to_string().color(c).on_color(on_c);

                print!("{}", s);
            }
            match y {
                0 => println!("   {}", self.p[0].get_name()),
                7 => println!("   {}", self.p[1].get_name()),
                _ => println!("")
            }
        }
        println!("");
    }
}
