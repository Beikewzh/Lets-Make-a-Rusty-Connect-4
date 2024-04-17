use std::fmt::{Display, Formatter, Result};
use strum_macros::EnumIter;

#[derive(EnumIter, Copy, Clone)]
pub enum Opponent {
	Human,
	EasyMode,
	NormalMode,
	ExpertMode,
}

impl Display for Opponent {
	/// Prints out the piece color
	fn fmt(&self, f: &mut Formatter) -> Result {
		match self {
			Opponent::Human => write!(f, "{}", "Human"),
			Opponent::EasyMode => write!(f, "{}", "Easy"),
			Opponent::NormalMode => write!(f, "{}", "Normal"),
			Opponent::ExpertMode => write!(f, "{}", "Expert"),
		}
	}
}

impl PartialEq for Opponent {
	fn eq(&self, other: &Opponent) -> bool {
		use Opponent::*;

		match (self, other) {
			(Human, Human) => true,
			(EasyMode, EasyMode) => true,
			(NormalMode, NormalMode) => true,
			(ExpertMode, ExpertMode) => true,
			_ => false,
		}
	}
}
