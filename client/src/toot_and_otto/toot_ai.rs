use super::{
	toot_and_otto::{TootAndOtto, Player::*, TOenum, TOenum::*},
};
use rand::{seq::SliceRandom, Rng};
use strum::IntoEnumIterator;

pub fn AI_next_move(board: TootAndOtto, difficulty: usize) -> (usize, TOenum) {
	// random move 
	let (rand_col, rand_letter) = random_move(board);
	
	// easy mode - random move
	if difficulty == 1 {
		return (rand_col, rand_letter);
	} 

	let mut rng = rand::thread_rng();
	let r: f64 = rng.gen_range(0.0,1.0);
	let (best_col, best_letter, _) = find_best_move(board, 3, true);

	// normal mode - 30% random move
	if difficulty == 2 {
		if r < 0.3 {
			return (rand_col, rand_letter);
		}
		else {
			return (best_col, best_letter);
		}
	}
	// expert
	return (best_col, best_letter);
	
	
}

pub fn random_move(board: TootAndOtto) -> (usize, TOenum) {
	loop {
		let rand_col = *board.get_columns().choose(&mut rand::thread_rng()).unwrap();
		let rand_letter =  *[O, T].choose(&mut rand::thread_rng()).unwrap();
		let mut clone_board = board.clone();
		if clone_board.drop(rand_letter, rand_col) {
			return (rand_col, rand_letter);
		}
	}
}

fn find_best_move(board: TootAndOtto, depth: usize, is_cpu_turn: bool) -> (usize, TOenum, i32) {
	if board.termination {
		return match board.winner {
			None => (3, O, 0), // Draw
			Some(player) => match player {
				OTTO => (3, O, i32::MAX - 25 + depth as i32), // Computer won, good
				TOOT => (3, O, i32::MIN + 25 - depth as i32), // Human won, bad
			},
		};
	} else if depth == 0 {
		return (3, O, board.heuristic_searc_score(OTTO));
	}

	if is_cpu_turn {
		let mut best_options = vec![(0, T, i32::MIN)];

		for letter in TOenum::iter() {
			for col in board.get_columns().iter() {
				let mut copy_board = board.clone();

				if copy_board.drop(letter, *col) == false {
					continue;
				}

				let new_value = find_best_move(copy_board, depth - 1, false).2;

				if new_value == best_options[0].2 {
					best_options.push((*col, letter, new_value));
				} else if new_value > best_options[0].2 {
					best_options = vec![(*col, letter, new_value)];
				}
			}
		}

		return *best_options.choose(&mut rand::thread_rng()).unwrap();
	} else {
		let mut best_options = vec![(0, T, i32::MAX)];

		for letter in TOenum::iter() {
			for col in board.get_columns().iter() {
				let mut copy_board = board.clone();
				if copy_board.drop(letter, *col) == false {
					continue;
				}

				let new_value = find_best_move(copy_board, depth - 1, true).2;

				if new_value == best_options[0].2 {
					best_options.push((*col, letter, new_value));
				} else if new_value < best_options[0].2 {
					best_options = vec![(*col, letter, new_value)];
				}
			}
		}

		return *best_options.choose(&mut rand::thread_rng()).unwrap();
	}
}
