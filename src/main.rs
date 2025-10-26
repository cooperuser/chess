use crossterm::cursor::MoveTo;
use crossterm::execute;
use crossterm::terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen};
use pleco::Board;
use std::fmt::format;
use std::io;
use std::io::prelude::*;

mod board;
mod chess;

fn main() -> io::Result<()> {
    execute!(io::stdout(), EnterAlternateScreen)?;
    // enable_raw_mode()?;

    let stdin = io::stdin();
    let mut board = Board::start_pos();
    let mut moves = Vec::new();
    board::print(&board, (1, 0))?;
    execute!(io::stdout(), MoveTo(0, 20))?;
    println!("{}", board.fen());
    execute!(io::stdout(), MoveTo(0, 22))?;

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let res = match line.as_str() {
            "" => break,
            "undo" => {
                board.undo_move();
                true
            }
            mv => {
                if board.apply_uci_move(mv) {
                    moves.push(line);
                }
                true
            }
        };

        execute!(io::stdout(), Clear(ClearType::All))?;
        board::print(&board, (1, 0))?;
        execute!(io::stdout(), MoveTo(0, 20))?;
        println!("{}", board.fen());
        execute!(io::stdout(), MoveTo(0, 21))?;
        println!("{}", if res { "" } else { "Illegal move" });
        execute!(io::stdout(), MoveTo(0, 23))?;
        print_moves(&moves)?;
        execute!(io::stdout(), MoveTo(0, 22))?;
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

    println!("{}", parts.join("  "));
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
