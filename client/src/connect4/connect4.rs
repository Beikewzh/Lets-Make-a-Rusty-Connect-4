use std::fmt::{Display, Formatter, Result};

use C4Piece::*;


/// The two pieces that can be placed on a Connect 4 board
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum C4Piece {
	P1,
	P2,
}

impl C4Piece {
	/// Flips the value of the piece
	pub fn switch(&self) -> Self {
		match self {
			C4Piece::P1 => C4Piece::P2,
			C4Piece::P2 => C4Piece::P1,
		}
	}
}

impl Display for C4Piece {
	/// Prints out the piece color
	fn fmt(&self, f: &mut Formatter) -> Result {
		match self {
			C4Piece::P1 => write!(f, "{}", "Red"),
			C4Piece::P2 => write!(f, "{}", "Yellow"),
		}
	}
}


#[derive(Copy, Clone)]
pub struct Connect4 {

	pub board: Board,

	pub current_player: C4Piece,

	pub winner: Option<C4Piece>,

	pub termination: bool,

	pub next_step: usize,

	pub col_row_index: [usize; NUM_COLS],
}

type Board = [[BoardCell; NUM_COLS]; NUM_ROWS];

pub const NUM_COLS: usize = 7;
pub const NUM_ROWS: usize = 6;

type BoardCell = Option<C4Piece>;

impl Connect4 {
	pub fn initialize() -> Self 
	{
		Connect4 
		{
			board: [[None; NUM_COLS]; NUM_ROWS],

			next_step: 0,

			current_player: P1,

			termination: false,

			winner: None,
			
			col_row_index: [0; NUM_COLS],
		}
	}

	pub fn get_columns(&self) -> [usize; NUM_COLS] 
	{
		return [3, 2, 4, 1, 5, 0, 6];
	}

	pub fn get_availiable_columns(&self) -> Vec<usize> {
		let mut vec = Vec::new();
		for i in self.get_columns() {
			if self.col_row_index[i] < NUM_ROWS {
				vec.push(i);
			}
		}
		vec

	}

	pub fn place(&mut self, col: usize) -> bool 
	{

		if col >= NUM_COLS 
		{
			return false;
		}

		let height = self.col_row_index[col];

		if height == NUM_ROWS 
		{
			return false;
		}
		else
		{
			let row = NUM_ROWS - 1 - height;

			self.board[row][col] = Some(self.current_player);

			self.next_step += 1;

			self.winner = match self.check_win(self.current_player) 
			{
				Some(_) => Some(self.current_player),
				None => None,
			};

			self.termination = self.winner != None || self.next_step == NUM_COLS * NUM_ROWS;

			self.col_row_index[col] += 1;

			self.current_player = self.current_player.switch();

			return true;
		}
	}

	pub fn check_win(&self, color: C4Piece) -> Option<Vec<[usize; 2]>> 
	{
		let part_checking = |partition: &[BoardCell]| -> bool 
		{
			for index in partition.iter() 
			{
				match index 
				{
					Some(c) => {if *c != color {return false;}}
					None => return false,
				}
			}
			return true;
		};

		for row_index in 0..NUM_ROWS 
		{
			for st_index in 0..NUM_COLS - 3 
			{
				let partition = &self.board[row_index][st_index..st_index + 4];
				if part_checking(partition) 
				{
					return Some((0..4).into_iter().map(|i| [row_index, st_index + i]).collect());
				}
			}
		}

		for col_index in 0..NUM_COLS 
		{
			for st_index in 0..NUM_ROWS - 3 
			{
				let mut partition = vec![];
				(st_index..st_index + 4).into_iter().for_each(|row_index| partition.push(self.board[row_index][col_index]));
				if part_checking(&partition) 
				{
					return Some((0..4).into_iter().map(|i| [st_index + i, col_index]).collect());
				}
			}
		}

		for row_index in NUM_ROWS - 3..NUM_ROWS 
		{
			for col_index in 0..NUM_COLS - 3 
			{
				let mut partition = vec![];
				(0..4).into_iter().for_each(|i| partition.push(self.board[row_index - i][col_index + i]));
				if part_checking(&partition) 
				{
					return Some((0..4).into_iter().map(|i| [row_index - i, col_index + i]).collect());
				}
			}
		}

		for row_index in 0..NUM_ROWS - 3 {
			for col_index in 0..NUM_COLS - 3 {
				let mut window = vec![];
				(0..4).into_iter().for_each(|i| window.push(self.board[row_index + i][col_index + i]));

				if part_checking(&window) 
				{
					return Some((0..4).into_iter().map(|i| [row_index + i, col_index + i]).collect());
				}
			}
		}
		
		return None;
	}

	pub fn heuristic_searc_score(&self, color: C4Piece) -> i32 
	{
		let mut score = 0;

		let partition_score = |partition: &[(BoardCell, bool)]| -> i32 
		{
			let mut mine_chess = 0;
			let mut op_chess = 0;
			let mut below = 0;
			let mut empty = 0;

			for index in partition.iter() 
			{
				match index.0 
				{
					Some(own_color) => 
					{
						if own_color == color 
						{
							mine_chess += 1;
						} 
						else 
						{
							op_chess += 1;
						}
					},
					None => match index.1 
					{
						true => empty += 1,
						false => below += 1,
					}
				}
			}

			if mine_chess > 0 && op_chess > 0 
			{
				return 0;
			}
			else{

				if mine_chess == 4{
					return 1000000;
				}
				else if mine_chess == 3{
					return 50;
				}
				else if mine_chess == 2{
					return 2;
				}
				else if op_chess == 3 && empty == 1 && mine_chess == 0{
					return -1000000;
				}
				else if op_chess == 3 && below == 1 && mine_chess == 0{
					return -100;
				}
				else if op_chess == 2 && mine_chess == 0{
					return -10;
				}
				else{
					return 0;
				}
			}
		};

		for row_index in 0..NUM_ROWS 
		{
			for st_index in 0..NUM_COLS - 3 
			{
				let mut window: Vec<(BoardCell, bool)> = vec![];
				(st_index..st_index + 4).into_iter().for_each(|col| {window.push((
					self.board[row_index][col],
					self.col_row_index[col] >= NUM_ROWS - row_index - 1,))
				});
				score += partition_score(&window);
			}
		}

		for col_index in 0..NUM_COLS 
		{
			for st_index in 0..NUM_ROWS - 3 
			{
				let mut window: Vec<(BoardCell, bool)> = vec![];
				(st_index..st_index + 4).into_iter().for_each(|row| window.push((self.board[row][col_index], true)));
				score += partition_score(&window);
			}
		}

		for row_index in NUM_ROWS - 3..NUM_ROWS 
		{
			for col_index in 0..NUM_COLS - 3 
			{
				let mut window: Vec<(BoardCell, bool)> = vec![];
				(0..4).into_iter().for_each(|i| {window.push((
					self.board[row_index - i][col_index + i],
					self.col_row_index[col_index + i] >= NUM_ROWS - (row_index - i) - 1,))
				});
				score += partition_score(&window);
			}
		}

		for row_index in 0..NUM_ROWS - 3 
		{
			for col_index in 0..NUM_COLS - 3 
			{
				let mut window = vec![];
				(0..4).into_iter().for_each(|i| {window.push((
					self.board[row_index + i][col_index + i],
					self.col_row_index[col_index + i] >= NUM_ROWS - (row_index + i) - 1,))
				});
				score += partition_score(&window);
			}
		}


		for row_index in 0..NUM_ROWS 
		{
			match self.board[row_index][3] {
				Some(own_color) =>
				{
					if own_color == color 
					{
						score += 10;
					}
				},
				None => {}
			}
		}

		return score;
	}
}


impl Display for Connect4 
{

	fn fmt(&self, f: &mut Formatter) -> Result 
	{
		let mut printing = String::new();

		for row in 0..NUM_ROWS 
		{
			for col in 0..NUM_COLS 
			{
				match self.board[row][col] 
				{
					None => printing.push('.'),
					Some(color) => match color
					{
						P1 => printing.push('\u{25CF}'),
						P2 => printing.push('\u{25CB}'),
					},
				};

				printing.push(' ');
			}

			printing.push('\n');
		}

		printing.push_str("0 1 2 3 4 5 6");

		write!(f, "\nCurrent Board:\n{}\n", printing)
	}
}
