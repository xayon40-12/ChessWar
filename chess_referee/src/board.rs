use crate::Movement;
use colored::*;

pub struct Board {
    mat: [[char; 8]; 8],
    last_movement: Movement
}


impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let str: String = self.mat.iter().flat_map(|l| l.iter().chain(['\n'].iter())).collect();
        write!(f, "{}", str)
    }
}

impl Board {
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

        Board { mat, last_movement: Movement { before: (0,0), after: (0,0) } }
    }

    pub fn to_contiguous(&self) -> String {
       self.mat.iter().flat_map(|l| l.iter()).collect()
    }

    pub fn check_movement(&self, mov: &Movement, is_white: bool) -> bool {
        let (ax,ay) = mov.before;
        let (bx,by) = mov.after;
        let pa = self.mat[ay][ax];
        let pb = self.mat[by][bx];
        let dir = mov.dir();
        if pa == ' ' { 
            println!("Try to move empty square!"); return false; 
        }
        if (is_white && ('A'..='Z').contains(&pa)) || (!is_white && ('a'..='z').contains(&pa)) {
            println!("Try to move opponent piece!"); return false;
        }
        if (is_white && ('a'..='z').contains(&pb)) || (!is_white && ('A'..='Z').contains(&pb)) {
            println!("Try to arrive on friend piece!");
        }
        match pa {
            'p' => (dir == (0,1) && pb == ' ') || ((dir == (1,1) || dir == (-1,1)) && pb != ' ') ,
            'P' => (dir == (0,-1) && pb == ' ') || ((dir == (1,-1) || dir == (-1,-1)) && pb != ' ') ,
            ' ' => { println!("Cannot move empty square!"); false }
            _ => panic!("Unrecognised character while checking movement!")
        }
    }

    // try to apply the movement and return true if possible, false if wrong
    pub fn apply_movement(&mut self, mov: Movement, is_white: bool) -> bool {
        if self.check_movement(&mov, is_white) {
            self.mat[mov.after.1][mov.after.0] = self.mat[mov.before.1][mov.before.0];
            self.mat[mov.before.1][mov.before.0] = ' ';
            self.last_movement = mov;

            true
        } else {
            self.last_movement = mov;

            false
        }
    }

    pub fn show(&self) {
        for y in 0..8 {
            for x in 0..8 {
                let s = self.mat[y][x].to_string().red();
                let s = if (x+y)%2 == 0 { s.on_white() } else { s.on_black() };
                print!("{}", s);
            }
            println!("");
        }
    }
}
