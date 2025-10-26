use arboard::Clipboard;
use crossterm::cursor::MoveTo;
use crossterm::execute;
use crossterm::terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen};
use pleco::tools::Searcher;
use pleco::{Board, bot_prelude};
use std::io;
use std::io::prelude::*;

mod board;
mod chess;

fn main() -> io::Result<()> {
    let mut clipboard = Clipboard::new().unwrap();
    execute!(io::stdout(), EnterAlternateScreen)?;
    // enable_raw_mode()?;

    let stdin = io::stdin();
    let mut board = Board::start_pos();
    let mut moves = Vec::new();
    print_screen(&board, &moves, &mut clipboard, true)?;

    for line in stdin.lock().lines() {
        execute!(io::stdout(), Clear(ClearType::All))?;

        let line = line.unwrap();
        let success = match line.as_str() {
            "" => break,
            "undo" => {
                if board.moves_played() != 0 {
                    board.undo_move();
                    moves.pop();
                    true
                } else {
                    false
                }
            }
            "?" => {
                let possible = board.generate_moves().len();
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
                let mv = bot_prelude::IterativeSearcher::best_move(board.shallow_clone(), 5);
                board.apply_move(mv);
                moves.push(mv.stringify());
                true
            }
            mv => {
                if board.apply_uci_move(mv) {
                    moves.push(line);
                    true
                } else {
                    false
                }
            }
        };

        print_screen(&board, &moves, &mut clipboard, success)?;
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
    board: &Board,
    moves: &[String],
    clipboard: &mut Clipboard,
    success: bool,
) -> io::Result<()> {
    clipboard.set_text(board.fen()).unwrap();
    board::print(board, (1, 0))?;

    execute!(io::stdout(), MoveTo(0, 20))?;
    println!("{}", board.fen());
    execute!(io::stdout(), MoveTo(0, 21))?;
    println!("{}", if success { "" } else { "Illegal move" });
    execute!(io::stdout(), MoveTo(0, 24))?;
    print_moves(moves)?;

    execute!(io::stdout(), MoveTo(40, 1))?;
    if board.checkmate() {
        print!("{:?} is checkmated!", board.turn());
    } else if board.in_check() {
        print!("{:?} is in check!", board.turn());
    }

    execute!(io::stdout(), MoveTo(0, 22))?;
    Ok(())
}

fn print_moves(moves: &[String]) -> io::Result<()> {
    let parts: Vec<String> = moves
        .chunks(2)
        .enumerate()
        .map(|(index, set)| {
            format!(
                "{}. {} {}",
                index + 1,
                set[0],
                set.get(1).unwrap_or(&String::from(""))
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
