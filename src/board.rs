use std::io;

use crossterm::style::{Color, ResetColor, SetBackgroundColor, SetForegroundColor};
use crossterm::terminal::{Clear, ClearType};
use crossterm::{cursor::MoveTo, execute};
use timecat::prelude::*;

pub fn print(
    board: &Board,
    last: Option<Move>,
    next: Option<Move>,
    pos: (u16, u16),
) -> io::Result<()> {
    print_board(pos)?;
    if let Some(m) = last {
        print_move(m, pos, false)?;
    }
    if let Some(m) = next {
        print_move(m, pos, true)?;
    }
    print_pieces(board, pos)?;
    print_material(board, pos)?;
    execute!(io::stdout(), ResetColor)?;
    Ok(())
}

pub fn print_board(pos: (u16, u16)) -> io::Result<()> {
    execute!(io::stdout(), SetForegroundColor(Color::Yellow))?;
    execute!(io::stdout(), MoveTo(pos.0 + 4, pos.1))?;
    println!("a   b   c   d   e   f   g   h");
    execute!(io::stdout(), SetForegroundColor(Color::DarkGrey))?;
    execute!(io::stdout(), MoveTo(pos.0 + 2, pos.1 + 1))?;
    println!("▗▄▄▄▖   ▗▄▄▄▖   ▗▄▄▄▖   ▗▄▄▄▖    ");
    for i in 0..3 {
        execute!(io::stdout(), MoveTo(pos.0 + 2, pos.1 + i * 4 + 2))?;
        println!("▐███▌   ▐███▌   ▐███▌   ▐███▌    ");
        execute!(io::stdout(), MoveTo(pos.0 + 2, pos.1 + i * 4 + 3))?;
        println!("▝▀▀▀▚▄▄▄▞▀▀▀▚▄▄▄▞▀▀▀▚▄▄▄▞▀▀▀▚▄▄▄▖");
        execute!(io::stdout(), MoveTo(pos.0 + 2, pos.1 + i * 4 + 4))?;
        println!("    ▐███▌   ▐███▌   ▐███▌   ▐███▌");
        execute!(io::stdout(), MoveTo(pos.0 + 2, pos.1 + i * 4 + 5))?;
        println!("▗▄▄▄▞▀▀▀▚▄▄▄▞▀▀▀▚▄▄▄▞▀▀▀▚▄▄▄▞▀▀▀▘");
    }
    execute!(io::stdout(), MoveTo(pos.0 + 2, pos.1 + 14))?;
    println!("▐███▌   ▐███▌   ▐███▌   ▐███▌    ");
    execute!(io::stdout(), MoveTo(pos.0 + 2, pos.1 + 15))?;
    println!("▝▀▀▀▚▄▄▄▞▀▀▀▚▄▄▄▞▀▀▀▚▄▄▄▞▀▀▀▚▄▄▄▖");
    execute!(io::stdout(), MoveTo(pos.0 + 2, pos.1 + 16))?;
    println!("    ▐███▌   ▐███▌   ▐███▌   ▐███▌");
    execute!(io::stdout(), MoveTo(pos.0 + 2, pos.1 + 17))?;
    println!("    ▝▀▀▀▘   ▝▀▀▀▘   ▝▀▀▀▘   ▝▀▀▀▘");
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

pub fn print_move(m: Move, pos: (u16, u16), next: bool) -> io::Result<()> {
    let source = m.get_source();
    let source = (source.get_file().to_index(), source.get_rank().to_index());
    let source = (source.0 as u16, 7 - source.1 as u16);
    let dest = m.get_dest();
    let dest = (dest.get_file().to_index(), dest.get_rank().to_index());
    let dest = (dest.0 as u16, 7 - dest.1 as u16);
    let (source_fg, source_bg) = match (next, (source.0 + source.1) % 2) {
        (true, 0) => (Color::Yellow, Color::DarkGrey),
        (true, _) => (Color::Yellow, Color::Black),
        (_, 0) => (Color::Black, Color::DarkGrey),
        _ => (Color::DarkGrey, Color::Black),
    };
    let (dest_fg, dest_bg) = match (next, (dest.0 + dest.1) % 2) {
        (true, 0) => (Color::Yellow, Color::DarkGrey),
        (true, _) => (Color::Yellow, Color::Black),
        (_, 0) => (Color::Black, Color::DarkGrey),
        _ => (Color::DarkGrey, Color::Black),
    };
    execute!(
        io::stdout(),
        MoveTo(pos.0 + source.0 * 4 + 3, pos.1 + source.1 * 2 + 2)
    )?;
    execute!(io::stdout(), SetForegroundColor(source_fg))?;
    execute!(io::stdout(), SetBackgroundColor(source_bg))?;
    print!("[ ]");
    execute!(
        io::stdout(),
        MoveTo(pos.0 + dest.0 * 4 + 3, pos.1 + dest.1 * 2 + 2)
    )?;
    execute!(io::stdout(), SetForegroundColor(dest_fg))?;
    execute!(io::stdout(), SetBackgroundColor(dest_bg))?;
    print!("[ ]");
    Ok(())
}

pub fn print_pieces(board: &Board, pos: (u16, u16)) -> io::Result<()> {
    for y in 0..8u16 {
        for x in 0..8u16 {
            let sq = Square::from_rank_and_file(
                Rank::from_index(7 - y as usize),
                File::from_index(x as usize),
            );
            let piece = board
                .get_piece_type_at(sq)
                .map(piece_to_str)
                .unwrap_or_default();
            let foreground = match board.color_at(sq) {
                Some(timecat::Color::Black) => Color::Green,
                _ => Color::White,
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

pub fn print_material(board: &Board, pos: (u16, u16)) -> io::Result<()> {
    let mut materials = Vec::new();
    let mut scores: Vec<isize> = Vec::new();
    for c in 0..2 {
        let color = timecat::Color::from_index(c);
        let mut total_score = 0;
        let mut material: Vec<String> = Vec::new();
        for (piece, count, score) in [
            (PieceType::Pawn, 8, 1),
            (PieceType::Knight, 2, 3),
            (PieceType::Bishop, 2, 3),
            (PieceType::Rook, 2, 5),
            (PieceType::Queen, 1, 9),
        ] {
            let pieces = board.get_colored_piece_mask(piece, color).count();
            let pieces = count - pieces;
            if pieces > 0 {
                total_score += score * pieces;
                let pieces = [piece_to_str(piece)].repeat(pieces).join("");
                material.push(pieces);
            }
        }

        scores.push(total_score as isize);
        materials.push(material);
    }

    execute!(io::stdout(), ResetColor)?;
    for c in 0..2 {
        let color = if c == 0 { Color::White } else { Color::Green };
        execute!(io::stdout(), SetForegroundColor(color))?;
        execute!(io::stdout(), MoveTo(pos.0 + 40, pos.1 + 14 + 2 * c as u16))?;
        execute!(io::stdout(), Clear(ClearType::UntilNewLine))?;
        print!("{}", materials[1 - c].join(" "));
        let score = scores[1 - c] - scores[c];
        if score > 0 {
            execute!(io::stdout(), SetForegroundColor(Color::DarkGrey))?;
            print!(" +{score}");
        }
    }
    Ok(())
}

fn piece_to_str(piece: PieceType) -> &'static str {
    match piece {
        PieceType::Pawn => "󰡙",
        PieceType::Knight => "󰡘",
        PieceType::Bishop => "󰡜",
        PieceType::Rook => "󰡛",
        PieceType::Queen => "󰡚",
        PieceType::King => "󰡗",
    }
}
