use std::io;

use crossterm::style::{Color, ResetColor, SetBackgroundColor, SetForegroundColor};
use crossterm::{cursor::MoveTo, execute};
use pleco::{Board, Player};

use crate::chess;

pub fn print(board: &Board, pos: (u16, u16)) -> io::Result<()> {
    print_board(pos)?;
    print_pieces(board, pos)?;
    execute!(io::stdout(), ResetColor)?;
    Ok(())
}

pub fn print_board(pos: (u16, u16)) -> io::Result<()> {
    execute!(io::stdout(), SetForegroundColor(Color::Yellow))?;
    execute!(io::stdout(), MoveTo(pos.0 + 4, pos.1))?;
    println!("a   b   c   d   e   f   g   h");
    execute!(io::stdout(), SetForegroundColor(Color::DarkGrey))?;
    execute!(io::stdout(), MoveTo(pos.0 + 2, pos.1 + 1))?;
    println!("▗▄▄▄▖   ▗▄▄▄▖   ▗▄▄▄▖   ▗▄▄▄▖");
    for i in 0..3 {
        execute!(io::stdout(), MoveTo(pos.0 + 2, pos.1 + i * 4 + 2))?;
        println!("▐███▌   ▐███▌   ▐███▌   ▐███▌");
        execute!(io::stdout(), MoveTo(pos.0 + 2, pos.1 + i * 4 + 3))?;
        println!("▝▀▀▀▚▄▄▄▞▀▀▀▚▄▄▄▞▀▀▀▚▄▄▄▞▀▀▀▚▄▄▄▖");
        execute!(io::stdout(), MoveTo(pos.0 + 6, pos.1 + i * 4 + 4))?;
        println!("▐███▌   ▐███▌   ▐███▌   ▐███▌");
        execute!(io::stdout(), MoveTo(pos.0 + 2, pos.1 + i * 4 + 5))?;
        println!("▗▄▄▄▞▀▀▀▚▄▄▄▞▀▀▀▚▄▄▄▞▀▀▀▚▄▄▄▞▀▀▀▘");
    }
    execute!(io::stdout(), MoveTo(pos.0 + 2, pos.1 + 14))?;
    println!("▐███▌   ▐███▌   ▐███▌   ▐███▌");
    execute!(io::stdout(), MoveTo(pos.0 + 2, pos.1 + 15))?;
    println!("▝▀▀▀▚▄▄▄▞▀▀▀▚▄▄▄▞▀▀▀▚▄▄▄▞▀▀▀▚▄▄▄▖");
    execute!(io::stdout(), MoveTo(pos.0 + 6, pos.1 + 16))?;
    println!("▐███▌   ▐███▌   ▐███▌   ▐███▌");
    execute!(io::stdout(), MoveTo(pos.0 + 6, pos.1 + 17))?;
    println!("▝▀▀▀▘   ▝▀▀▀▘   ▝▀▀▀▘   ▝▀▀▀▘");
    execute!(io::stdout(), SetForegroundColor(Color::Yellow))?;
    execute!(io::stdout(), MoveTo(pos.0 + 4, pos.1 + 18))?;
    println!("a   b   c   d   e   f   g   h");
    for y in 0..8 {
        execute!(io::stdout(), MoveTo(pos.0, pos.1 + y * 2 + 2))?;
        print!("{}", 8 - y);
        execute!(io::stdout(), MoveTo(pos.0 + 36, pos.1 + y * 2 + 2))?;
        print!("{}", 8 - y);
    }

    // println!("    ▗▄▄▄▖   ▗▄▄▄▖   ▗▄▄▄▖   ▗▄▄▄▖");
    // for _ in 0..3 {
    //     println!("    ▐███▌   ▐███▌   ▐███▌   ▐███▌");
    //     println!("▗▄▄▄▞▀▀▀▚▄▄▄▞▀▀▀▚▄▄▄▞▀▀▀▚▄▄▄▞▀▀▀▘");
    //     println!("▐███▌   ▐███▌   ▐███▌   ▐███▌");
    //     println!("▝▀▀▀▚▄▄▄▞▀▀▀▚▄▄▄▞▀▀▀▚▄▄▄▞▀▀▀▚▄▄▄▖");
    // }
    // println!("    ▐███▌   ▐███▌   ▐███▌   ▐███▌");
    // println!("▗▄▄▄▞▀▀▀▚▄▄▄▞▀▀▀▚▄▄▄▞▀▀▀▚▄▄▄▞▀▀▀▘");
    // println!("▐███▌   ▐███▌   ▐███▌   ▐███▌");
    // println!("▝▀▀▀▘   ▝▀▀▀▘   ▝▀▀▀▘   ▝▀▀▀▘");

    Ok(())
}

pub fn print_pieces(board: &Board, pos: (u16, u16)) -> io::Result<()> {
    for y in 0..8u16 {
        for x in 0..8u16 {
            let sq = chess::pos_to_sq(7 - y as usize, x as usize);
            let (player, piece) = match board.piece_at_sq(sq) {
                pleco::Piece::None => (Player::White, " "),
                pleco::Piece::WhitePawn => (Player::White, "󰡙"),
                pleco::Piece::WhiteKnight => (Player::White, "󰡘"),
                pleco::Piece::WhiteBishop => (Player::White, "󰡜"),
                pleco::Piece::WhiteRook => (Player::White, "󰡛"),
                pleco::Piece::WhiteQueen => (Player::White, "󰡚"),
                pleco::Piece::WhiteKing => (Player::White, "󰡗"),
                pleco::Piece::BlackPawn => (Player::Black, "󰡙"),
                pleco::Piece::BlackKnight => (Player::Black, "󰡘"),
                pleco::Piece::BlackBishop => (Player::Black, "󰡜"),
                pleco::Piece::BlackRook => (Player::Black, "󰡛"),
                pleco::Piece::BlackQueen => (Player::Black, "󰡚"),
                pleco::Piece::BlackKing => (Player::Black, "󰡗"),
            };
            let foreground = match player {
                Player::White => Color::White,
                Player::Black => Color::Green,
            };
            let background = match (x + y) % 2 {
                0 => Color::DarkGrey,
                _ => Color::Black,
            };
            execute!(io::stdout(), MoveTo(pos.0 + x * 4 + 4, pos.1 + y * 2 + 2))?;
            execute!(io::stdout(), SetForegroundColor(foreground))?;
            execute!(io::stdout(), SetBackgroundColor(background))?;
            print!("{}", piece);
        }
    }
    Ok(())
}
