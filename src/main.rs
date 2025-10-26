use arboard::Clipboard;
use crossterm::cursor::{self, MoveTo};
use crossterm::event::{KeyCode, KeyModifiers, read};
use crossterm::execute;
use crossterm::style::{Color, SetForegroundColor};
use crossterm::terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use timecat::prelude::*;

use std::io;

mod board;

fn main() -> io::Result<()> {
    let mut clipboard = Clipboard::new().unwrap();
    execute!(io::stdout(), EnterAlternateScreen)?;
    enable_raw_mode()?;

    let mut board = Board::default();
    let mut word: Vec<String> = Vec::new();
    let mut history: Vec<Vec<String>> = Vec::new();
    let mut index = 0usize;
    print_screen(&board, &word, &mut clipboard)?;

    let halt = loop {
        let Ok(event) = read() else {
            continue;
        };
        let Some(event) = event.as_key_press_event() else {
            continue;
        };

        match (event.modifiers, event.code) {
            (_, KeyCode::Esc) => break false,
            (KeyModifiers::CONTROL, KeyCode::Char('c')) => break true,
            (KeyModifiers::CONTROL, KeyCode::Char('l')) => {
                execute!(io::stdout(), Clear(ClearType::All))?;
                word.clear();
            }
            (KeyModifiers::CONTROL, KeyCode::Char('b')) => {
                if board.generate_legal_moves().is_empty() {
                    continue;
                }
                let mut engine = Engine::from_board(board.shallow_clone());
                let response = engine.search_depth_quiet(6);
                let best_move = response.get_best_move().expect("No best move");
                _ = board.push(best_move);
            }
            (_, KeyCode::Up) => {
                index = (index + 1).min(history.len());
                word = match history.get(history.len() - index) {
                    Some(w) => w.clone(),
                    _ => Vec::new(),
                };
            }
            (_, KeyCode::Down) => {
                index = index.saturating_sub(1);
                word = match history.get(history.len() - index) {
                    Some(w) => w.clone(),
                    _ => Vec::new(),
                };
            }
            (_, KeyCode::Backspace) => {
                word.pop();
            }
            (_, KeyCode::Char(c)) => {
                word.push(c.to_string());
            }
            (_, KeyCode::Enter) => match word.join("").as_str() {
                "undo" => {
                    _ = board.pop();
                    add_to_history(&mut history, &mut word, &mut index);
                }
                "?" => {
                    let possible = board.generate_legal_moves().len();
                    execute!(io::stdout(), MoveTo(40, 2))?;
                    print!(
                        "{:?} has {} possible move{}",
                        board.turn(),
                        possible,
                        if possible != 1 { "s" } else { "" }
                    );
                    add_to_history(&mut history, &mut word, &mut index);
                }
                "bot" => {
                    if board.generate_legal_moves().is_empty() {
                        continue;
                    }
                    let mut engine = Engine::from_board(board.shallow_clone());
                    let response = engine.search_depth_quiet(6);
                    let best_move = response.get_best_move().expect("No best move");
                    _ = board.push(best_move);
                    add_to_history(&mut history, &mut word, &mut index);
                }
                m => {
                    if board.push_san(m).is_ok() {
                        add_to_history(&mut history, &mut word, &mut index);
                    }
                }
            },
            _ => {}
        }

        print_screen(&board, &word, &mut clipboard)?;
    };

    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;

    std::process::exit(if halt { 130 } else { 0 });
}

fn print_screen(board: &Board, word: &[String], clipboard: &mut Clipboard) -> io::Result<()> {
    clipboard.set_text(board.get_fen()).unwrap();

    execute!(io::stdout(), MoveTo(0, 22))?;
    execute!(io::stdout(), cursor::Hide)?;
    execute!(io::stdout(), Clear(ClearType::CurrentLine))?;
    let next = match word.join("").as_str() {
        "" | "?" | "bot" | "undo" => {
            execute!(io::stdout(), SetForegroundColor(Color::Blue))?;
            println!("‣");
            None
        }
        word => {
            let m: Result<Move, TimecatError> = board.parse_san(word);
            match m {
                Ok(m) => {
                    execute!(io::stdout(), SetForegroundColor(Color::Green))?;
                    println!("󰸞");
                    Some(m)
                }
                Err(_) => {
                    execute!(io::stdout(), SetForegroundColor(Color::Red))?;
                    println!("×");
                    None
                }
            }
        }
    };

    execute!(io::stdout(), MoveTo(40, 3))?;
    execute!(io::stdout(), Clear(ClearType::CurrentLine))?;
    if board.is_stalemate() {
        print!("{:?} is stalemated!", board.turn());
    } else if board.is_checkmate() {
        print!("{:?} is checkmated!", board.turn());
    } else if board.is_check() {
        print!("{:?} is in check!", board.turn());
    }

    let last = board.get_last_stack_move();
    board::print(board, last.map(|m| m.unwrap()), next, (1, 2))?;

    execute!(io::stdout(), MoveTo(0, 0))?;
    execute!(io::stdout(), Clear(ClearType::CurrentLine))?;
    println!("{}", board.get_fen());
    execute!(io::stdout(), MoveTo(0, 24))?;
    println!("Moves played: {}", format_moves(board));

    execute!(io::stdout(), MoveTo(2, 22))?;
    println!("{}", word.join(""));
    execute!(io::stdout(), MoveTo(2 + word.len() as u16, 22))?;
    execute!(io::stdout(), cursor::Show)?;
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

fn add_to_history(history: &mut Vec<Vec<String>>, word: &mut Vec<String>, index: &mut usize) {
    if *index == 0 {
        history.push(word.clone());
    }
    word.clear();
    *index = 0;
}
