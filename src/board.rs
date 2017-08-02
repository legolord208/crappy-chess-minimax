use *;

macro_rules! white {
	($name:ident) => {
		Piece::$name(false)
	}
}
macro_rules! black {
	($name:ident) => {
		Piece::$name(true)
	}
}
macro_rules! none {
	() => {
		Piece::Empty
	}
}

pub type Board = [[Piece; 8]; 8];

pub fn board_string(board: &Board) -> String {
	let mut output = String::with_capacity(8 * 9 + (9 + 2 + 1)); // width * height + (height + index + newline)

	output.push_str("  ABCDEFGH\n");
	for (i, row) in board.iter().enumerate() {
		output.push_str(&(i + 1).to_string());
		output.push(' ');
		for col in row {
			output.push(col.to_char());
		}
		output.push('\n');
	}

	output
}

pub fn make_board() -> Board {
	[
		[ white!(Rook), white!(Knight), white!(Bishop), white!(King), white!(Queen), white!(Bishop), white!(Knight), white!(Rook) ],
		[ white!(Pawn), white!(Pawn),   white!(Pawn),   white!(Pawn), white!(Pawn),  white!(Pawn),   white!(Pawn),   white!(Pawn) ],
		[ none!(),      none!(),        none!(),        none!(),      none!(),       none!(),        none!(),        none!()      ],
		[ none!(),      none!(),        none!(),        none!(),      none!(),       none!(),        none!(),        none!()      ],
		[ none!(),      none!(),        none!(),        none!(),      none!(),       none!(),        none!(),        none!()      ],
		[ none!(),      none!(),        none!(),        none!(),      none!(),       none!(),        none!(),        none!()      ],
		[ black!(Pawn), black!(Pawn),   black!(Pawn),   black!(Pawn), black!(Pawn),  black!(Pawn),   black!(Pawn),   black!(Pawn) ],
		[ black!(Rook), black!(Knight), black!(Bishop), black!(King), black!(Queen), black!(Bishop), black!(Knight), black!(Rook) ],
	]
}
