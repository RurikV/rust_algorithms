fn king_moves(position: u8) -> (u8, u64) {
    let row = position / 8;
    let col = position % 8;
    let mut moves: u64 = 0;
    let directions = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),          (0, 1),
        (1, -1), (1, 0), (1, 1)
    ];

    for (dr, dc) in directions.iter() {
        let new_row = row as i8 + dr;
        let new_col = col as i8 + dc;
        if (0..8).contains(&new_row) && (0..8).contains(&new_col) {
            let new_position = (new_row * 8 + new_col) as u8;
            moves |= 1 << new_position;
        }
    }

    let move_count = moves.count_ones() as u8;
    (move_count, moves)
}

fn knight_moves(position: u8) -> (u8, u64) {
    let knight_bits = 1u64 << position;
    
    // Define constants for invalid moves at board edges
    const NA: u64 = 0xFEFEFEFEFEFEFEFE;
    const NAB: u64 = 0xFCFCFCFCFCFCFCFC;
    const NH: u64 = 0x7F7F7F7F7F7F7F7F;
    const NGH: u64 = 0x3F3F3F3F3F3F3F3F;

    // Calculate possible moves
    let moves = NGH & (knight_bits <<  6 | knight_bits >> 10)
             |  NH & (knight_bits << 15 | knight_bits >> 17)
             |  NA & (knight_bits << 17 | knight_bits >> 15)
             | NAB & (knight_bits << 10 | knight_bits >>  6);
    
    let move_count = moves.count_ones() as u8;
    (move_count, moves)
}

fn rook_moves(position: u8) -> (u8, u64) {
    let row = position / 8;
    let col = position % 8;
    let mut moves: u64 = 0;

    // Horizontal moves
    for c in 0..8 {
        if c != col {
            moves |= 1 << (row * 8 + c);
        }
    }

    // Vertical moves
    for r in 0..8 {
        if r != row {
            moves |= 1 << (r * 8 + col);
        }
    }

    let move_count = moves.count_ones() as u8;
    (move_count, moves)
}

fn bishop_moves(position: u8) -> (u8, u64) {
    let row = position / 8;
    let col = position % 8;
    let mut moves: u64 = 0;

    // Top-left diagonal
    for i in 1..8 {
        let new_row = row as i8 - i;
        let new_col = col as i8 - i;
        if new_row >= 0 && new_col >= 0 {
            moves |= 1 << (new_row * 8 + new_col) as u8;
        } else {
            break;
        }
    }

    // Top-right diagonal
    for i in 1..8 {
        let new_row = row as i8 - i;
        let new_col = col as i8 + i;
        if new_row >= 0 && new_col < 8 {
            moves |= 1 << (new_row * 8 + new_col) as u8;
        } else {
            break;
        }
    }

    // Bottom-left diagonal
    for i in 1..8 {
        let new_row = row as i8 + i;
        let new_col = col as i8 - i;
        if new_row < 8 && new_col >= 0 {
            moves |= 1 << (new_row * 8 + new_col) as u8;
        } else {
            break;
        }
    }

    // Bottom-right diagonal
    for i in 1..8 {
        let new_row = row as i8 + i;
        let new_col = col as i8 + i;
        if new_row < 8 && new_col < 8 {
            moves |= 1 << (new_row * 8 + new_col) as u8;
        } else {
            break;
        }
    }

    let move_count = moves.count_ones() as u8;
    (move_count, moves)
}

fn queen_moves(position: u8) -> (u8, u64) {
    let (_rook_move_count, rook_moves) = rook_moves(position);
    let (_bishop_move_count, bishop_moves) = bishop_moves(position);
    let combined_moves = rook_moves | bishop_moves;
    let move_count = combined_moves.count_ones() as u8;
    (move_count, combined_moves)
}

fn count_set_bits_kernighan(mut n: u64) -> u32 {
    let mut count = 0;
    while n != 0 {
        n &= n - 1; // Clear the least significant set bit
        count += 1;
    }
    count
}

const TABLE: [u32; 256] = [
    0, 1, 1, 2, 1, 2, 2, 3, 1, 2, 2, 3, 2, 3, 3, 4, 1, 2, 2, 3, 2, 3, 3, 4, 2, 3, 3, 4, 3, 4, 4, 5, 
    1, 2, 2, 3, 2, 3, 3, 4, 2, 3, 3, 4, 3, 4, 4, 5, 2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6, 
    1, 2, 2, 3, 2, 3, 3, 4, 2, 3, 3, 4, 3, 4, 4, 5, 2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6, 
    2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6, 3, 4, 4, 5, 4, 5, 5, 6, 4, 5, 5, 6, 5, 6, 6, 7, 
    1, 2, 2, 3, 2, 3, 3, 4, 2, 3, 3, 4, 3, 4, 4, 5, 2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6, 
    2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6, 3, 4, 4, 5, 4, 5, 5, 6, 4, 5, 5, 6, 5, 6, 6, 7, 
    2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6, 3, 4, 4, 5, 4, 5, 5, 6, 4, 5, 5, 6, 5, 6, 6, 7, 
    3, 4, 4, 5, 4, 5, 5, 6, 4, 5, 5, 6, 5, 6, 6, 7, 4, 5, 5, 6, 5, 6, 6, 7, 5, 6, 6, 7, 6, 7, 7, 8
];

fn count_set_bits_table(n: u64) -> u32 {
    TABLE[(n & 0xff) as usize] 
        + TABLE[((n >> 8) & 0xff) as usize]
        + TABLE[((n >> 16) & 0xff) as usize]
        + TABLE[((n >> 24) & 0xff) as usize]
        + TABLE[((n >> 32) & 0xff) as usize]
        + TABLE[((n >> 40) & 0xff) as usize]
        + TABLE[((n >> 48) & 0xff) as usize]
        + TABLE[((n >> 56) & 0xff) as usize]
}

use std::collections::HashMap;

fn count_set_bits_cached(n: u64, cache: &mut HashMap<u64, u32>) -> u32 {
    if let Some(&count) = cache.get(&n) {
        return count;
    }

    let count = if n == 0 {
        0
    } else {
        count_set_bits_cached(n & (n - 1), cache) + 1 // Kernighan's algorithm
    };

    cache.insert(n, count);
    count
}


use std::io;

#[allow(dead_code)]
fn main() {
    let num = 0b101101011; // Example number

    let count_kernighan = count_set_bits_kernighan(num);
    println!("Kernighan's Algorithm: {}", count_kernighan);

    let count_table = count_set_bits_table(num);
    println!("Lookup Table: {}", count_table);

    let mut cache = HashMap::new();

    let count = count_set_bits_cached(num, &mut cache);
    println!("Cached: {}", count);

    println!("Choose the piece (king/knight/rook/bishop/queen):");
    let mut piece = String::new();
    io::stdin().read_line(&mut piece).expect("Failed to read line");

    println!("Enter the position (from 0 to 63):");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let position: u8 = input.trim().parse().expect("Invalid input");

    match piece.trim().to_lowercase().as_str() {
        "king" => {
            let (moves_count, moves_bits) = king_moves(position);
            println!("Number of possible moves: {}", moves_count);
            println!("Bitboard of possible moves: {}", moves_bits);
        },
        "knight" => {
            let (moves_count, moves_bits) = knight_moves(position);
            println!("Number of possible moves: {}", moves_count);
            println!("Bitboard of possible moves: {}", moves_bits);
        },
        "rook" => {
            let (moves_count, moves_bits) = rook_moves(position);
            println!("Number of possible moves: {}", moves_count);
            println!("Bitboard of possible moves: {}", moves_bits);
        },
        "bishop" => {
            let (moves_count, moves_bits) = bishop_moves(position);
            println!("Number of possible moves: {}", moves_count);
            println!("Bitboard of possible moves: {}", moves_bits);
        },
        "queen" => {
            let (moves_count, moves_bits) = queen_moves(position);
            println!("Number of possible moves: {}", moves_count);
            println!("Bitboard of possible moves: {}", moves_bits);
        },
        _ => println!("Invalid piece selected. Please choose either 'king', 'knight', 'rook', 'bishop', or 'queen'."),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_king_moves() {
        let test_cases = [
            (0, 3, 770),
            (1, 5, 1797),
            (7, 3, 49216),
            (8, 5, 197123),
            (10, 8, 920078),
            (15, 5, 12599488),
            (54, 8, 16186183351374184448),
            (55, 5, 13853283560024178688),
            (56, 3, 144959613005987840),
            (63, 3, 4665729213955833856),
        ];

        for (input, expected_move_count, expected_moves) in test_cases.iter() {
            let (move_count, moves) = king_moves(*input);
            assert_eq!(move_count, *expected_move_count);
            assert_eq!(moves, *expected_moves);
        }
    }

    #[test]
    fn test_knight_moves() {
        let test_cases = [
            (0, 2, 132096),
            (1, 3, 329728),
            (2, 4, 659712),
            (36, 8, 11333767002587136),
            (47, 4, 4620693356194824192),
            (48, 3, 288234782788157440),
            (54, 4, 1152939783987658752),
            (55, 3, 2305878468463689728),
            (56, 2, 1128098930098176),
            (63, 2, 9077567998918656)
        ];

        for (input, expected_move_count, expected_moves) in test_cases.iter() {
            let (move_count, moves) = knight_moves(*input);
            assert_eq!(move_count, *expected_move_count);
            assert_eq!(moves, *expected_moves);
        }
    }

    #[test]
    fn test_rook_moves() {
        let test_cases = [
            (0, 14, 72340172838076926),
            (1, 14, 144680345676153597),
            (2, 14, 289360691352306939),
            (36, 14, 1157443723186933776),
            (47, 14, 9259541023762186368),
            (48, 14, 143553341945872641),
            (54, 14, 4665518383679160384),
            (55, 14, 9259260648297103488),
            (56, 14, 18302911464433844481),
            (63, 14, 9187484529235886208)
        ];

        for (input, expected_move_count, expected_moves) in test_cases.iter() {
            let (move_count, moves) = rook_moves(*input);
            assert_eq!(move_count, *expected_move_count);
            assert_eq!(moves, *expected_moves);
        }
    }

    #[test]
    fn test_bishop_moves() {
        let test_cases = [
            (0, 7, 9241421688590303744),
            (1, 7, 36099303471056128),
            (2, 7, 141012904249856),
            (36, 13, 9386671504487645697),
            (47, 7, 2323857683139004420),
            (48, 7, 144117404414255168),
            (54, 9, 11529391036782871041),
            (55, 7, 4611756524879479810),
            (56, 7, 567382630219904),
            (63, 7, 18049651735527937)
        ];

        for (input, expected_move_count, expected_moves) in test_cases.iter() {
            let (move_count, moves) = bishop_moves(*input);
            assert_eq!(move_count, *expected_move_count);
            assert_eq!(moves, *expected_moves);
        }
    }


    #[test]
    fn test_queen_moves() {
        let test_cases = [
            (0, 21, 9313761861428380670),
            (1, 21, 180779649147209725),
            (2, 21, 289501704256556795),
            (36, 27, 10544115227674579473),
            (47, 21, 11583398706901190788),
            (48, 21, 287670746360127809),
            (54, 23, 16194909420462031425),
            (55, 21, 13871017173176583298),
            (56, 21, 18303478847064064385),
            (63, 21, 9205534180971414145)
        ];

        for (input, expected_move_count, expected_moves) in test_cases.iter() {
            let (move_count, moves) = queen_moves(*input);
            assert_eq!(move_count, *expected_move_count);
            assert_eq!(moves, *expected_moves);
        }
    }
}
