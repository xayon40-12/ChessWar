use crate::{Movement,Player};
use colored::*;
use std::env;

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

fn initialise_players() -> [Player; 2] {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 { panic!("Too few arguments!"); }

    //first player white 'b' and second black 'n'
    [Player::new(&args[1],'b'), Player::new(&args[2],'n')]
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

        let p = initialise_players();

        Board { mat, last_movement: Movement { before: (0,0), after: (0,0) }, p, i: 0 }
    }

    pub fn to_contiguous(&self) -> String {
       self.mat.iter().flat_map(|l| l.iter()).collect()
    }

    pub fn check_movement(mat: &[[char; 8]; 8], mov: &Movement, is_white: bool) -> bool {
        let (ax,ay) = mov.before;
        let (bx,by) = mov.after;
        let pa = mat[ay][ax];
        let pb = mat[by][bx];
        let (dir,udir) = (mov.dir(),mov.udir());
        if mov.before == mov.after {
            println!("No movement done!"); return false;
        }
        if pa == ' ' { 
            println!("Try to move empty square!"); return false; 
        }
        if (is_white && ('A'..='Z').contains(&pa)) || (!is_white && ('a'..='z').contains(&pa)) {
            println!("Try to move opponent piece!"); return false;
        }
        if (is_white && ('a'..='z').contains(&pb)) || (!is_white && ('A'..='Z').contains(&pb)) {
            println!("Try to arrive on friend piece!");
        }

        let prop = |n| (0..n).fold(((ax as i8,ay as i8),true), |(p,r),_| ((p.0+udir.0,p.1+udir.1),mat[p.1 as usize][p.0 as usize] == ' ' && r)).1;
        let rock = || false; //TODO
        match pa {
            'p' => (dir == (0,1) && pb == ' ') || ((dir == (1,1) || dir == (-1,1)) && pb != ' ') ,
            'P' => (dir == (0,-1) && pb == ' ') || ((dir == (1,-1) || dir == (-1,-1)) && pb != ' ') ,
            't' | 'T' => udir.0.abs()+udir.1.abs() == 1 && prop(dir.0.abs()+dir.1.abs()),
            'f' | 'F' => dir.0.abs() == dir.1.abs() && prop(dir.0.abs()),
            'd' | 'D' => (dir.0.abs() == dir.1.abs() && prop(dir.0.abs())) || (udir.0.abs()+udir.1.abs() == 1 && prop(dir.0.abs()+dir.1.abs())),
            'r' | 'R' => dir == udir || rock(),
            'c' | 'C' => dir.0.abs()*dir.1.abs() == 2,
            ' ' =>  false,
            _ => panic!("Unrecognised character while checking movement!")
        }
    }

    // try to apply the movement and return true if possible, false if wrong
    pub fn apply_movement(&mut self, mov: Movement, is_white: bool) -> bool {
        if Board::check_movement(&self.mat, &mov, is_white) {
            self.mat[mov.after.1][mov.after.0] = self.mat[mov.before.1][mov.before.0];
            self.mat[mov.before.1][mov.before.0] = ' ';
            self.last_movement = mov;

            true
        } else {
            self.last_movement = mov;

            false
        }
    }

    pub fn targetable(&self, (x,y): (usize,usize)) -> bool {
        let pa = self.mat[y][x];
        if pa == ' ' { return true; }
        let white = ('a'..='z').contains(&pa);
        for j in 0..8 {
            for i in 0..8 {
                if white != ('a'..='z').contains(&self.mat[j][i]) && self.mat[j][i] != ' ' {
                    if Board::check_movement(&self.mat, &Movement { before: (x,y), after: (i,j) }, white) {
                        return true;
                    }
                }
            }
        }

        false
    }

    pub fn check(&self) -> bool {
        let r = if self.p[self.i].is_white() { 'r' } else { 'R' };
        for j in 0..8 {
            for i in 0..8 {
                if self.mat[j][i] == r {                        
                    return self.targetable((i,j));             
                }
            }
        }
        
        true
    }

    pub fn turn(&mut self) {
        let (i,j) = (self.i,1-self.i);
        let m = self.p[i].next_move(self.to_contiguous());
        if !self.apply_movement(m, self.p[i].is_white()) {
            println!("Player {} has made a mistake!", self.p[i].get_name());
            self.show(true);
            self.p[i].kill();
            self.p[j].kill();
            std::process::exit(0);
        }
        self.show(false);
        self.i = 1-i;
    }

    pub fn show(&self, mistake: bool) {
        println!("{}'s turn:", self.p[self.i].get_name());
        for y in 0..8 {
            for x in 0..8 {
                let c = if self.last_movement.after == (x,y) && !mistake { "green" } else { if (x+y)%2 == 0 { "black" } else { "white" } };
                let on_c = if self.last_movement.before == (x,y) { "green" } else { 
                    if mistake && self.last_movement.after == (x,y) { "red" } else { if (x+y)%2 == 0 { "white" } else { "black" } } 
                };
                let s = self.mat[y][x].to_string().color(c).on_color(on_c);

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
