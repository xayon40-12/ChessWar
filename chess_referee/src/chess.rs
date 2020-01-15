use crate::Movement;

pub const WHITE_PIECES: [char; 6] = ['p','t','c','f','r','d'];
pub const BLACK_PIECES: [char; 6] = ['P','T','C','F','R','D'];

pub fn check_movement(mat: &[[char; 8]; 8], mov: &Movement) -> Result<(),String> {
    let (ax,ay) = mov.before;
    let (bx,by) = mov.after;
    let pa = mat[ay][ax];
    let pb = mat[by][bx];
    let (dir,udir) = (mov.dir(),mov.udir());
    if mov.before == mov.after {
        return Err("No movement done!".into());
    }
    if pa == ' ' { 
        return Err("Try to move empty square!".into());
    }
    if WHITE_PIECES.contains(&pa) ^ BLACK_PIECES.contains(&pb) && pb != ' ' {
        return Err("Try to arrive on friend piece!".into());
    }
    let prop = |n| (0..n).fold(((ax as i8,ay as i8),true), |(p,r),_| ((p.0+udir.0,p.1+udir.1),mat[p.1 as usize][p.0 as usize] == ' ' && r)).1;
    let rock = || false; //TODO
    if match pa {
        'p' => (dir == (0,1) && pb == ' ') || ((dir == (1,1) || dir == (-1,1)) && pb != ' ') ,
        'P' => (dir == (0,-1) && pb == ' ') || ((dir == (1,-1) || dir == (-1,-1)) && pb != ' ') ,
        't' | 'T' => udir.0.abs()+udir.1.abs() == 1 && prop(dir.0.abs()+dir.1.abs()),
        'f' | 'F' => dir.0.abs() == dir.1.abs() && prop(dir.0.abs()),
        'd' | 'D' => (dir.0.abs() == dir.1.abs() && prop(dir.0.abs())) || (udir.0.abs()+udir.1.abs() == 1 && prop(dir.0.abs()+dir.1.abs())),
        'r' | 'R' => dir == udir || rock(),
        'c' | 'C' => dir.0.abs()*dir.1.abs() == 2,
        _ => panic!("Unrecognised character while checking movement!")
    } {
        Ok(())
    } else {
        return Err("Wrong movement!".into());
    }
}

pub fn check_player_movement(mat: &[[char; 8]; 8], mov: &Movement, is_white: bool) -> Result<(),String> {
    let (ax,ay) = mov.before;
    let pa = mat[ay][ax];
    if (is_white && BLACK_PIECES.contains(&pa)) || (!is_white && WHITE_PIECES.contains(&pa)) {
        return Err("Try to move opponent piece!".into());
    }

    check_movement(mat, mov)
}

// Search the pieces and return them with their position if they are still on the board.
// If 'pieces' is empty, search for all remaining pieces.
pub fn find(mat: &[[char; 8]; 8], pieces: &[char]) -> Vec<(usize,usize)> {
    let mut found = Vec::new();
    for j in 0..8 {
        for i in 0..8 {
            let c = mat[j][i];
            if c != ' ' {
                if pieces.contains(&c) || pieces.len() == 0 {    
                    found.push((i,j));
                }
            }
        }
    }

    found
}

// Verify if a square is targetable by any piece or if a piece is targable by an opponent
// piece and return each piece that do target.
pub fn targetable(mat: &[[char; 8]; 8], (x,y): (usize,usize)) -> Vec<(usize,usize)> {
    let pa = mat[y][x]; 
    let pieces = 
        if pa == ' '                       { find(mat, &[]) } 
        else if WHITE_PIECES.contains(&pa) { find(mat, &BLACK_PIECES) } 
        else                               { find(mat, &WHITE_PIECES) };
    pieces.into_iter().filter(
        |p| check_movement(mat, &Movement { before: (x,y), after: *p }).is_ok()
    ).collect()
}

pub fn check(mat: &[[char; 8]; 8], is_white: bool) -> Vec<(usize,usize)> {
    let p = find(mat, &vec![if is_white { 'r' } else { 'R' }])[0];
    targetable(mat, p)
}

pub fn pat(_mat: &[[char; 8]; 8]) -> bool {
    false //TODO
}

pub fn checkmate(mat: &[[char; 8]; 8], is_white: bool) -> bool {
    if check(mat, is_white).len() == 0 { return false }

    let friends = find(mat, if is_white { &WHITE_PIECES } else { &BLACK_PIECES });
    for p in friends {
        for j in 0..8 {
            for i in 0..8 {
                if check_movement(mat, &Movement { before: p, after: (i,j) }).is_ok() {
                    let mut tmp_mat = mat.clone();
                    tmp_mat[j][i] = tmp_mat[p.1][p.0];
                    tmp_mat[p.1][p.0] = ' ';
                    if check(&tmp_mat, is_white).len() == 0 { return false }
                }
            }
        }
    }

    true
}

