#[derive(Debug)]
pub struct Movement {
    pub before: (usize,usize),
    pub after: (usize,usize),
}

impl Movement {
    pub fn dir(&self) -> (i8,i8) {
        (self.after.0 as i8 - self.before.0 as i8, self.after.1 as i8 - self.before.1 as i8)
    }

    pub fn udir(&self) -> (i8,i8) {
        let (x,y) = self.dir();
        (x.signum(),y.signum())
    }
}

impl From<String> for Movement {
    fn from(line: String) -> Self {
        let line = line.trim();
        if line.len() != 4 { panic!("Wrong Movement string.") }
        let coord = line.chars().map(|c| c as i8).collect::<Vec<_>>();
        let before = ((coord[0] as i8 - 'a' as i8) as usize, (8-(coord[1] as i8 - '0' as i8)) as usize);
        let after = ((coord[2] as i8 - 'a' as i8) as usize, (8-(coord[3] as i8 - '0' as i8)) as usize);
        Movement { before, after }
    }
}

impl From<&Movement> for String {
    fn from(mov: &Movement) -> Self {
        let a = 'a' as u8;
        format!("{}{}{}{}", (mov.before.0 as u8 + a) as char, 8-mov.before.1,  (mov.after.0 as u8 + a) as char, 8-mov.after.1)
    }
}

impl std::fmt::Display for Movement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let str: String = self.into();
        write!(f, "{}", str)
    }
}


