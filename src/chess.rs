use pleco::{File, Rank, SQ};

pub fn pos_to_sq(rank: usize, file: usize) -> SQ {
    let rank = to_rank(rank);
    let file = to_file(file);
    SQ::make(file, rank)
}

pub fn to_file(file: usize) -> File {
    match file {
        0 => File::A,
        1 => File::B,
        2 => File::C,
        3 => File::D,
        4 => File::E,
        5 => File::F,
        6 => File::G,
        7 => File::H,
        _ => panic!("Invalid file: {file}"),
    }
}

pub fn to_rank(rank: usize) -> Rank {
    match rank {
        0 => Rank::R1,
        1 => Rank::R2,
        2 => Rank::R3,
        3 => Rank::R4,
        4 => Rank::R5,
        5 => Rank::R6,
        6 => Rank::R7,
        7 => Rank::R8,
        _ => panic!("Invalid rank: {rank}"),
    }
}
