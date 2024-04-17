use std::fmt::{Display, Formatter, Result};
use strum_macros::EnumIter;
use TOenum::*;
use Player::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, EnumIter)]
pub enum TOenum {
	T,
	O,
}


#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Player {
	TOOT,
	OTTO,
}

impl Player {
	pub fn switch(&self) -> Self {
		match self {
			TOOT => OTTO,
			OTTO => TOOT,
		}
	}
}

impl Display for Player {
	fn fmt(&self, f: &mut Formatter) -> Result {
		match self {
			TOOT => write!(f, "{}", "TOOT"),
			OTTO => write!(f, "{}", "OTTO"),
		}
	}
}

#[derive(Clone, Copy)]
pub struct TootAndOtto {
	pub board: Board,

	pub winner: Option<Player>,

	pub current_player: Player,

	pub termination: bool,

	pub next_step: usize,

	pub current_height: [usize; NUM_COLS],

	pub countings: [[usize; 2]; 2], 
}

type Board = [[BoardCell; NUM_COLS]; NUM_ROWS];

pub const NUM_COLS: usize = 6;
pub const NUM_ROWS: usize = 4;

type BoardCell = Option<TOenum>;

impl TootAndOtto {
	pub fn new() -> Self 
	{
		TootAndOtto 
		{
			board: [[None; NUM_COLS]; NUM_ROWS],

			current_height: [0; NUM_COLS],

			current_player: TOOT,

			termination: false,

			next_step: 0,

			winner: None,
			
			countings: [[6; 2]; 2],
		}
	}

	pub fn get_columns(&self) -> [usize; NUM_COLS] 
	{
		[2, 3, 1, 4, 0, 5]
	}

	pub fn drop(&mut self, letter: TOenum, col: usize) -> bool 
	{
		if col >= NUM_COLS 
		{
			return false;
		}

		let piece_count = match self.current_player 
		{
			TOOT => match letter 
			{
				T => self.countings[0][0],
				O => self.countings[0][1],
			},
			OTTO => match letter 
			{
				T => self.countings[1][0],
				O => self.countings[1][1],
			},
		};

		if piece_count == 0 
		{
			return false;
		}

		
		let height = self.get_height(col);

		if height == NUM_ROWS 
		{
			return false;
		}

		let row = NUM_ROWS - 1 - height;

		self.board[row][col] = Some(letter);

		let winning_TOOT = self.check_win(TOOT);
		let winning_OTTO = self.check_win(OTTO);

		self.winner = match (winning_TOOT, winning_OTTO) 
		{
			(Some(_), Some(_)) => {self.termination = true;None}

			(None, Some(_)) => {self.termination = true;Some(OTTO)}

			(Some(_), None) => {self.termination = true;Some(TOOT)}

			(None, None) => None,
		};

		match self.current_player 
		{
			TOOT => match letter 
			{
				T => self.countings[0][0] -= 1,
				O => self.countings[0][1] -= 1,
			},
			OTTO => match letter 
			{
				T => self.countings[1][0] -= 1,
				O => self.countings[1][1] -= 1,
			},
		}

		self.next_step += 1;

		if self.next_step == NUM_COLS * NUM_ROWS 
		{
			self.termination = true
		}

		self.current_player = self.current_player.switch();

		return true;
	}

	fn get_height(&self, col: usize) -> usize 
	{
		for row_index in 0..NUM_ROWS {
			match self.board[row_index][col] {
				None => {}
				_ => return NUM_ROWS - row_index,
			}
		}

		return 0;
	}

	pub fn check_win(&self, player: Player) -> Option<Vec<[usize; 2]>> 
	{
		let winning_condition = match player 
		{
			TOOT => [T, O, O, T],
			OTTO => [O, T, T, O],
		};

		let part_checking = |partition: &[BoardCell]| -> bool 
		{
			for (i, index) in partition.iter().enumerate() 
			{
				match index 
				{
					None => return false,
					Some(own_color) => 
					{
						if *own_color != winning_condition[i] 
						{
							return false;
						}
					}
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

		for col_index in 0..NUM_COLS - 3 
		{
			let mut partition = vec![];

			(0..4).into_iter().for_each(|i| partition.push(self.board[NUM_ROWS - 1 - i][col_index + i]));

			if part_checking(&partition) 
			{
				return Some((0..4).into_iter().map(|i| [NUM_ROWS - 1 - i, col_index + i]).collect(),);
			}
		}

		for col_index in 0..NUM_COLS - 3 
		{
			let mut partition = vec![];

			(0..4).into_iter().for_each(|i| partition.push(self.board[i][col_index + i]));

			if part_checking(&partition) 
			{
				return Some((0..4).into_iter().map(|i| [i, col_index + i]).collect(),);
			}
		}

		return None;
	}

	pub fn heuristic_searc_score(&self, player: Player) -> i32 
	{
	
		let winning_condition = match player {
			TOOT => [T, O, O, T],
			OTTO => [O, T, T, O],
		};

		let mut score = 0;

		let partition_score = |partition: &[(BoardCell, bool)]| -> i32 
		{

			let mut mine_chess = 0;
			let mut op_chess = 0;
			let mut below = 0;
			let mut empty = 0;

			for (i, j) in partition.iter().enumerate() 
			{
				match j.0 
				{
					Some(l) => 
					{
						if l == winning_condition[i] 
						{
							mine_chess += 1
						} 
						else 
						{
							op_chess += 1
						}
					},
					None => match partition[0].1 
					{
						false => below += 1,
						true => empty += 1,
					}
				}
			}

			if mine_chess > 0 && op_chess > 0 {
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
				let mut partition: Vec<(BoardCell, bool)> = vec![];
				(st_index..st_index + 4).into_iter().for_each(|col_index| {partition.push((
					self.board[row_index][col_index],
					self.current_height[col_index] >= NUM_ROWS - row_index - 1,))
				});

				score += partition_score(&partition);
			}
		}

		for col_index in 0..NUM_COLS 
		{
			for st_index in 0..NUM_ROWS - 3 
			{
				let mut partition: Vec<(BoardCell, bool)> = vec![];
				(st_index..st_index + 4).into_iter().for_each(|row_index| partition.push((self.board[row_index][col_index], true)));
				score += partition_score(&partition);
			}
		}

		for col_index in 0..NUM_COLS - 3 
		{
			let mut partition: Vec<(BoardCell, bool)> = vec![];

			(0..4).into_iter().for_each(|i| {partition.push((
				self.board[NUM_ROWS - 1 - i][col_index + i],
				self.current_height[col_index + i] >= i,))
			});

			score += partition_score(&partition);
		}

		for col_index in 0..NUM_COLS - 3 
		{
			let mut partition: Vec<(BoardCell, bool)> = vec![];

			(0..4).into_iter().for_each(|i| {partition.push((
				self.board[i][col_index + i],
				self.current_height[col_index + i] >= NUM_ROWS - i - 1,))
			});

			score += partition_score(&partition);
		}


		return score;
	}


}


impl Display for TOenum 
{
	fn fmt(&self, f: &mut Formatter) -> Result 
	{
		match self 
		{
			O => write!(f, "{}", "O"),
			T => write!(f, "{}", "T"),
		}
	}
}

impl Display for TootAndOtto 
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
					Some(color) => printing.push_str(format!("{}", color).as_str()),
				};

				printing.push(' ');
			}

			printing.push('\n');
		}

		printing.push_str("0 1 2 3 4 5");

		write!(f, "\nCurrent Board:\n{}\n", printing)
	}
}
