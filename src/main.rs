use std::io;

fn player_to_char(player: i32) -> char {
    if player == 1 {
        'X'
    } else if player == 2 {
        'O'
    } else {
        panic!("There are only two players, 1 and 2");
    }
}

fn to_char(board: &[i32; 9], pos: usize) -> char {
    if board[pos] == 0 && pos < 10 {
        ' '
    } else {
        player_to_char(board[pos])
    }
}

fn print_board(board: &[i32; 9]) {
    println!(
        " {} | {} | {}\n-----------\n \
       {} | {} | {}\n-----------\n \
       {} | {} | {}",
        to_char(&board, 0),
        to_char(&board, 1),
        to_char(&board, 2),
        to_char(&board, 3),
        to_char(&board, 4),
        to_char(&board, 5),
        to_char(&board, 6),
        to_char(&board, 7),
        to_char(&board, 8)
    )
}

fn print_line()
{
    println!("\n\n------------------------");
}

fn is_valid_pos(pos: usize) -> bool {
    pos <= 8
}

fn is_valid_move(board: &[i32; 9], pos: usize) -> bool {
    board[pos] == 0
}

fn make_move(board: &[i32; 9], pos: usize, player: i32) -> [i32; 9] {
    if player != 1 && player != 2 {
        panic!("player can only be 1 or 2");
    }

    let mut new_board = board.clone();
    new_board[pos] = player;
    return new_board;
}

fn is_victory(board: &[i32; 9], a: usize, b: usize, c: usize) -> bool {
    return board[a] > 0
        && board[b] > 0
        && board[c] > 0
        && board[a] == board[b]
        && board[b] == board[c];
}

fn is_game_over(board: &[i32; 9]) -> Option<i32> {
    // check vertical victory
    if is_victory(board, 0, 3, 6) {
        return Some(board[0]);
    }
    if is_victory(board, 1, 4, 7) {
        return Some(board[1]);
    }
    if is_victory(board, 2, 5, 8) {
        return Some(board[2]);
    }

    // check horizontal victory
    if is_victory(board, 0, 1, 2)
    {
        return Some(board[0]);
    }
    if is_victory(board, 3, 4, 5)
    {
        return Some(board[3]);
    }
    if is_victory(board, 6, 7, 8)
    {
        return Some(board[6]);
    }

    // check diagonal victory
    if is_victory(board, 0, 4, 8)
    {
        return Some(board[0]);
    }
    if is_victory(board, 2, 4, 6)
    {
        return Some(board[2]);
    }
    None
}

fn is_board_full(board: &[i32; 9]) -> bool {
    for x in board {
        if *x == 0 {
            return false;
        }
    }
    return true;
}

fn game_over_check(board: &[i32; 9]) -> bool
{
    if let Some(result) = is_game_over(board) {
        print_line();
        print_board(board);
        println!("Player {} won!!!", player_to_char(result));
        return true;
    }
    if is_board_full(board) {
        println!("Tie!");
        return true;
    }
    return false;
}

fn get_next_move(board: &[i32; 9]) -> usize
{
    print_board(board);
    println!("Please enter a position (0-8) for your move:");
    let mut pos = String::new();
    io::stdin()
        .read_line(&mut pos)
        .expect("Failed to read line");

    let pos: usize = pos.trim().parse().expect("Please type a positive number!");

    if is_valid_pos(pos) && is_valid_move(board, pos) {
        return pos;
    }

    println!("Please enter a valid move!\n\n");
    return get_next_move(board);
}

fn get_next_player(player: i32) -> i32
{
    if player == 2 {
        return 1;
    }
    return 2;
}

fn play_game(board: &[i32; 9], player: i32)
{
    print_line();
    println!("Player {} turn", player_to_char(player));

    let pos = get_next_move(board);
    let board = make_move(&board, pos, player);

    if game_over_check(&board) {
        return ();
    }
    let player = get_next_player(player);
    return play_game(&board, player);
}

fn main() {
    let board = [0; 9];
    play_game(&board, 1);
}
