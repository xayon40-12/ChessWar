use chess_referee::{chess,Movement};

fn get() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();

    line.trim().into()
}

fn get_info() -> (bool, bool) {
    let line = get();
    let v = line.split(" ").collect::<Vec<_>>();
    (v[0] == "b", v[1] == "1")
}

fn get_board() -> [[char; 8]; 8] {
    let mut board = [[' '; 8]; 8];
    let line = get();
    line.chars().enumerate().for_each(|(i,c)| board[(i/8) as usize][(i%8) as usize] = c); 
    board
}

fn rnd(len: usize) -> Vec<usize> {
    let mut r = Vec::new();
    while r.len() != len {
        let rand = rand::random::<usize>()%len;
        if !r.contains(&rand) { r.push(rand) }
    }

    r
}

fn main() {

    'l: loop {
        let (white,_rock) = get_info();
        let board = get_board();
        let pieces = chess::find(&board, if white { &chess::WHITE_PIECES } else { &chess::BLACK_PIECES });
        for before in rnd(pieces.len()).into_iter().map(|i| pieces[i]) {
            for after in rnd(8*8).into_iter().map(|i| (i%8,i/8)) {
                    if before == after { continue; }

                    let mov = Movement { before, after };
                    if chess::check_movement(&board, &mov) {
                        println!("{}", &mov);
                        continue'l;
                    }
            }
        }
    }
}
