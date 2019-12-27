fn get() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();

    line.trim().into()
}

enum Colour {
    Blanc,
    Noir
}

fn get_info() -> (Colour, bool) {
    let line = get();
    let v = line.split(" ").collect::<Vec<_>>();
    (if v[0] == "b" { Colour::Blanc } else { Colour::Noir }, if v[1] == "1" { true } else { false })
}

fn get_board() -> [[char; 8]; 8] {
    let mut board = [[' '; 8]; 8];
    let line = get();
    line.chars().enumerate().for_each(|(i,c)| board[(i/8) as usize][(i%8) as usize] = c); 
    board
}

fn print_move(before: (u8,u8), after: (u8,u8)) {
    let a = 'a' as u8;
    println!("{}{}{}{}", (before.0+a) as char, 8-before.1,  (after.0+a) as char, 8-after.1);
}

fn main() {
    loop {
        
        let (colour,_rock) = get_info();
        let mut _board = get_board();
        let (before,after) = match colour {
            Colour::Blanc => ((0,1),(0,2)),
            Colour::Noir => ((0,6),(0,5)),
        };

        print_move(before,after);
    }
}
