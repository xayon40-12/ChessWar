pub struct Board {
    mat: [[char; 8]; 8]
}

pub struct Movement {
    before: (usize,usize),
    after: (usize,usize)
}

impl From<[u8; 4]> for Movement {
    fn from(coord: [u8; 4]) -> Self {
        let before = ((coord[0]-'a' as u8) as usize, (8-coord[1]) as usize);
        let after = ((coord[2]-'a' as u8) as usize, (8-coord[3]) as usize);
        Movement { before, after }
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

        Board { mat }
    }

    pub fn to_contiguous(&self) -> String {
       self.mat.iter().flat_map(|l| l.iter()).collect()
    }

    // try to apply the movement and return true if possible, false if wrong
    pub fn apply_movement(&mut self, mov: Movement) -> bool {
        

        false
    }
}
