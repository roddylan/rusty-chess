
type PiecePos = u64;

static 

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



    return
}

fn main() {

}
