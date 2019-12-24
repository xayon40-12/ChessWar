
pub struct Movement {
    pub before: (usize,usize),
    pub after: (usize,usize),
}

impl Movement {
    pub fn dir(&self) -> (u8,u8) {
        (self.after.0 as u8 - self.before.0 as u8, self.after.1 as u8 - self.before.1 as u8)
    }
}

impl From<[u8; 4]> for Movement {
    fn from(coord: [u8; 4]) -> Self {
        let before = ((coord[0]-'a' as u8) as usize, (8-coord[1]) as usize);
        let after = ((coord[2]-'a' as u8) as usize, (8-coord[3]) as usize);
        Movement { before, after }
    }
}
