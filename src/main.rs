use arboard::Clipboard;
use crossterm::cursor::MoveTo;
use crossterm::execute;
use crossterm::terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen};
use timecat::prelude::*;

use std::io;
use std::io::prelude::*;

mod board;

fn main() -> io::Result<()> {
    let mut clipboard = Clipboard::new().unwrap();
    execute!(io::stdout(), EnterAlternateScreen)?;
    // enable_raw_mode()?;

    let stdin = io::stdin();
    let mut board = Board::default();
    print_screen(&board, &mut clipboard, true)?;

    for line in stdin.lock().lines() {
        execute!(io::stdout(), Clear(ClearType::All))?;

        let line = line.unwrap();
        let success = match line.as_str() {
            "" => break,
            "undo" => board.pop().is_ok(),
            "?" => {
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
            mv => board.push_san(mv).is_ok(),
        };

        print_screen(&board, &mut clipboard, success)?;
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

fn print_screen(board: &Board, clipboard: &mut Clipboard, success: bool) -> io::Result<()> {
    clipboard.set_text(board.get_fen()).unwrap();
    board::print(board, (1, 2))?;

    execute!(io::stdout(), MoveTo(0, 0))?;
    println!("{}", board.get_fen());
    execute!(io::stdout(), MoveTo(0, 22))?;
    println!("{}", if success { "" } else { "Illegal move" });
    execute!(io::stdout(), MoveTo(0, 25))?;
    println!("Moves played: {}", format_moves(board));

    execute!(io::stdout(), MoveTo(40, 3))?;
    if board.is_checkmate() {
        print!("{:?} is checkmated!", board.turn());
    } else if board.is_check() {
        print!("{:?} is in check!", board.turn());
    }

    execute!(io::stdout(), MoveTo(0, 23))?;
    Ok(())
}

fn format_moves(board: &Board) -> String {
    board
        .get_stack()
        .iter()
        .map(|(b, m)| m.algebraic(b, false).expect("Illegal move was played"))
        .collect::<Vec<_>>()
        .chunks(2)
        .enumerate()
        .map(|(index, pair)| format!("{}. {}", index + 1, pair.join(" ")))
        .collect::<Vec<_>>()
        .join("  ")
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
