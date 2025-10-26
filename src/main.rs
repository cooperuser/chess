use arboard::Clipboard;
use crossterm::cursor::MoveTo;
use crossterm::execute;
use crossterm::terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen};
use shakmaty::san::{ParseSanError, San, SanError};
use shakmaty::{Chess, Move, Position};
use std::io;
use std::io::prelude::*;

mod board;
mod chess;

fn main() -> io::Result<()> {
    let mut clipboard = Clipboard::new().unwrap();
    execute!(io::stdout(), EnterAlternateScreen)?;
    // enable_raw_mode()?;

    let stdin = io::stdin();
    let mut boards: Vec<Chess> = vec![Chess::default()];
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
                let chess = boards.last().unwrap();
                let possible = chess.legal_moves().len();
                execute!(io::stdout(), MoveTo(40, 2))?;
                print!(
                    "{:?} has {} possible move{}",
                    chess.turn(),
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
            mv => match mv.parse::<San>() {
                Ok(san) => {
                    let chess = boards.last().unwrap();
                    match san.to_move(chess) {
                        Ok(mv) => {
                            boards.push(chess.clone().play(mv).unwrap());
                            moves.push(mv);
                            true
                        }
                        Err(SanError::IllegalSan) => false,
                        Err(SanError::AmbiguousSan) => false,
                    }
                }
                Err(ParseSanError) => false,
            },
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
    boards: &[Chess],
    moves: &[Move],
    clipboard: &mut Clipboard,
    success: bool,
) -> io::Result<()> {
    let chess = boards.last().unwrap();
    let board = chess.board();
    clipboard.set_text(board.board_fen().to_string()).unwrap();
    board::print(board, (1, 2))?;

    execute!(io::stdout(), MoveTo(0, 0))?;
    println!("{}", board.board_fen());
    execute!(io::stdout(), MoveTo(0, 21))?;
    println!("{}", if success { "" } else { "Illegal move" });
    execute!(io::stdout(), MoveTo(0, 24))?;
    print_moves(boards, moves)?;

    execute!(io::stdout(), MoveTo(40, 3))?;
    if chess.is_checkmate() {
        print!("{:?} is checkmated!", chess.turn());
    } else if chess.is_check() {
        print!("{:?} is in check!", chess.turn());
    }

    execute!(io::stdout(), MoveTo(0, 22))?;
    Ok(())
}

fn print_moves(boards: &[Chess], moves: &[Move]) -> io::Result<()> {
    let parts: Vec<String> = moves
        .chunks(2)
        .enumerate()
        .map(|(index, set)| {
            format!(
                "{}. {}",
                index + 1,
                set.iter()
                    .enumerate()
                    .map(|m| { San::from_move(&boards[index / 2 + m.0], *m.1) })
                    .map(|m| m.to_string())
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
