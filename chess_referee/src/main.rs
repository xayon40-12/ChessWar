use std::env;
use chess_referee::Player;
use chess_referee::Board;

fn initialise_players() -> (Player,Player) {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 { panic!("Too few arguments!"); }

    //first player white 'b' and second black 'n'
    (Player::new(&args[1],'b'), Player::new(&args[2],'n'))
}

fn player_action(p1: &mut Player, p2: &mut Player, board: &mut Board) {
    let m = p1.next_move(&board);
    if !board.apply_movement(m, p1.is_white()) {
        println!("Player {} has made a mistake!", p1.get_name());
        p1.kill();
        p2.kill();
        std::process::exit(0);
    }
}

fn main() {
    let (mut p1, mut p2) = initialise_players();

    let mut board = Board::initialise_board();

    loop {
        player_action(&mut p1, &mut p2, &mut board);
        println!("{}", &board);
        player_action(&mut p2, &mut p1, &mut board);
        println!("{}", &board);
    }
}
