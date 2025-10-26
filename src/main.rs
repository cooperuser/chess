use arboard::Clipboard;
use crossterm::cursor::MoveTo;
use crossterm::execute;
use crossterm::terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen};
use std::io;
use std::io::prelude::*;
use timecat::prelude::*;

mod board;
// mod chess;

fn main() -> io::Result<()> {
    let mut clipboard = Clipboard::new().unwrap();
    execute!(io::stdout(), EnterAlternateScreen)?;
    // enable_raw_mode()?;

    let stdin = io::stdin();
    let mut boards: Vec<Board> = vec![Board::default()];
    let mut moves: Vec<Move> = Vec::new();
    print_screen(&boards, &moves, &mut clipboard, true)?;

    for line in stdin.lock().lines() {
        execute!(io::stdout(), Clear(ClearType::All))?;

        let line = line.unwrap();
        let success = match line.as_str() {
            "" => break,
            "undo" => {
                if !moves.is_empty() {
                    boards.pop();
                    moves.pop();
                    true
                } else {
                    false
                }
            }
            "?" => {
                let board = boards.last().unwrap();
                let possible = board.generate_legal_moves().len();
                execute!(io::stdout(), MoveTo(40, 2))?;
                print!(
                    "{:?} has {} possible move{}",
                    board.turn(),
                    possible,
                    if possible != 1 { "s" } else { "" }
                );
                true
            }
            "bot" => {
                // let mv = bot_prelude::IterativeSearcher::best_move(board.shallow_clone(), 5);
                // board.apply_move(mv);
                // moves.push(mv.stringify());
                true
            }
            mv => {
                let mut board = boards.last().unwrap().clone();
                if let Ok(valid) = board.push_san(mv) {
                    let mv = valid.unwrap();
                    boards.push(board);
                    moves.push(mv);
                    // match valid {
                    //     Some(_) => todo!(),
                    //     None => todo!(),
                    // }
                    //     boards.push(board);
                    //     moves.push(mv);
                    // }
                    true
                } else {
                    false
                }
            } // mv => match mv.parse::<San>() {
              //     Ok(san) => {
              //         let chess = boards.last().unwrap();
              //         match san.to_move(chess) {
              //             Ok(mv) => {
              //                 boards.push(chess.clone().play(mv).unwrap());
              //                 moves.push(mv);
              //                 true
              //             }
              //             Err(SanError::IllegalSan) => false,
              //             Err(SanError::AmbiguousSan) => false,
              //         }
              //     }
              //     Err(ParseSanError) => false,
              // },
        };

        print_screen(&boards, &moves, &mut clipboard, success)?;
    }

    // while let Ok(event) = read() {
    //     let Some(event) = event.as_key_press_event() else {
    //         continue;
    //     };
    //     if event.code == KeyCode::Esc {
    //         break;
    //     }
    // }

    // disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;
    Ok(())
}

fn print_screen(
    boards: &[Board],
    moves: &[Move],
    clipboard: &mut Clipboard,
    success: bool,
) -> io::Result<()> {
    let board = boards.last().unwrap();
    clipboard.set_text(board.get_fen()).unwrap();
    board::print(board, (1, 2))?;

    execute!(io::stdout(), MoveTo(0, 0))?;
    println!("{}", board.get_fen());
    execute!(io::stdout(), MoveTo(0, 22))?;
    println!("{}", if success { "" } else { "Illegal move" });
    execute!(io::stdout(), MoveTo(0, 25))?;
    print_moves(boards, moves)?;

    execute!(io::stdout(), MoveTo(40, 3))?;
    if board.is_checkmate() {
        print!("{:?} is checkmated!", board.turn());
    } else if board.is_check() {
        print!("{:?} is in check!", board.turn());
    }

    execute!(io::stdout(), MoveTo(0, 23))?;
    Ok(())
}

fn print_moves(boards: &[Board], moves: &[Move]) -> io::Result<()> {
    let parts: Vec<String> = moves
        .chunks(2)
        .enumerate()
        .map(|(index, set)| {
            format!(
                "{}. {}",
                index + 1,
                set.iter()
                    .enumerate()
                    .map(|m| {
                        m.1.algebraic(&boards[index * 2 + m.0], false)
                            .expect("Illegal move was played")
                    })
                    .collect::<Vec<_>>()
                    .join(" ")
            )
        })
        .collect();

    println!("Moves played: {}", parts.join("  "));
    Ok(())
}

// fn print_events() -> io::Result<()> {
//     while let Ok(event) = read() {
//         let Some(event) = event.as_key_press_event() else {
//             continue;
//         };
//         let modifier = match event.modifiers {
//             KeyModifiers::NONE => "".to_string(),
//             _ => format!("{:}+", event.modifiers),
//         };
//         println!("Key pressed: {modifier}{code}\r", code = event.code);
//         if event.code == KeyCode::Esc {
//             break;
//         }
//     }
//     Ok(())
// }
