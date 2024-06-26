
type PiecePos = u64;

static COL_MAP: [char; 8] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'
];

static LOOKUP67: [usize; 67+1] = [
    64, 0, 1, 39, 2, 15, 40, 23,
    3, 12, 16, 59, 41, 19, 24, 54,
    4, 64, 13, 10, 17, 62, 60, 28,
    42, 30, 20, 51, 25, 44, 55, 47,
    5, 32, 64, 38, 14, 22, 11, 58,
    18, 53, 63, 9, 61, 27, 29, 50,
    43, 46, 31, 37, 21, 57, 52, 8,
    26, 49, 45, 36, 56, 7, 48, 35,
    6, 34, 33, 64
];


// Piece
enum Color {
    White,
    Black
}

enum PieceType {
    Pawn,
    Rook, 
    Knight,
    Bishop,
    Queen,
    King
}


struct Piece {
    pos: PiecePos,
    color: Color,
    p_type: PieceType
}

// Square
enum Square {
    Empty,
    Occupied(usize)
}

// Game type
struct Game {
    pieces: Vec<Piece>,
    squares: Vec<Square>
}

impl Game {
    fn to_string(&self) -> String {
        let mut board = "".to_owned();
    }
}


fn bit_to_pos(bit: PiecePos) -> Result<String, String> {
    if bit == 0 {
        return Err("Invalid Piece; No piece present".to_string());
    }
    else {
        let idx = bit_scan(bit);
        return Ok(i_to_pos(idx));
    }
}

fn i_to_pos(idx: usize) -> String {
    let col = idx % 8;
    let row = idx / 8 + 1;
    return format!("{}{}", COL_MAP[col], row)
}

fn bit_scan(bit: u64) -> usize {
    let r = (bit % 67) as usize;
    return LOOKUP67[r];
}


fn main() {
    println!("hello world");
    println!("{}", LOOKUP67[67]);
}


