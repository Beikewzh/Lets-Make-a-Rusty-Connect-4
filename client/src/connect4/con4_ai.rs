use super::connect4::{Connect4, C4Piece::*};
use rand::{seq::SliceRandom, Rng};

pub fn AI_next_move(board: Connect4, difficulty: usize) -> usize 
{
	// random move 
	let rand_col = random_move(board);
	
	// easy mode - random move
	if difficulty == 1 {
		return rand_col;
	} 

	let mut rng = rand::thread_rng();
	let r: f64 = rng.gen_range(0.0,1.0);
	let (best_col, _) = find_best_move(board, 3, true);

	// normal mode - 30% random move
	if difficulty == 2 {
		if r < 0.3 {
			return rand_col;
		}
		else {
			return best_col;
		}
	}
	
	// expert
	return best_col;
	
}

pub fn random_move(board: Connect4) -> usize {
	*board.get_availiable_columns().choose(&mut rand::thread_rng()).unwrap()
}

// Refer to
// https://medium.com/analytics-vidhya/artificial-intelligence-at-play-connect-four-minimax-algorithm-explained-3b5fc32e4a4f
fn find_best_move(board: Connect4, depth: usize, is_cpu_turn: bool) -> (usize, i32) 
{
	if board.termination 
	{
		return match board.winner 
		{
			None => (3, 0), 
			Some(color) => match color 
			{
				P2 => (3, (i32::MAX - 43 + depth as i32)), 
				P1 => (3, i32::MIN + 43 - depth as i32),      
			},
		};
	} 
	else if depth == 0 
	{
		return (3, board.heuristic_searc_score(P2));
	}

	if is_cpu_turn 
	{
		let mut best_options = vec![(0, i32::MIN)];

		for col in board.get_columns().iter() 
		{
			let mut copy_board = board.clone();

			if copy_board.place(*col) == false 
			{
				continue;
			}
			let new_value = find_best_move(copy_board, depth - 1, false).1;

			if new_value == best_options[0].1 
			{
				best_options.push((*col, new_value));
			} 
			else if new_value > best_options[0].1 
			{
				best_options = vec![(*col, new_value)];
			}
		}

		*best_options.choose(&mut rand::thread_rng()).unwrap()
	} 
	else 
	{
		let mut best_options = vec![(0, i32::MAX)];

		for col in board.get_columns().iter() 
		{
			let mut copy_board = board.clone();
			if copy_board.place(*col) == false 
			{
				continue;
			}

			let new_value = find_best_move(copy_board, depth - 1, true).1;

			if new_value == best_options[0].1 
			{
				best_options.push((*col, new_value));
			} 
			else if new_value < best_options[0].1 
			{
				best_options = vec![(*col, new_value)];
			}
		}
		*best_options.choose(&mut rand::thread_rng()).unwrap()
	}
}