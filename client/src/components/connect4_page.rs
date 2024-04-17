use crate::{
	connect4::{
		connect4::{Connect4, NUM_COLS, NUM_ROWS, C4Piece, C4Piece::*},
		con4_ai,
	},
	types::opponent::Opponent,
};
use serde_json::json;
use strum::IntoEnumIterator;
use yew::format::Json;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct Connect4Page {
	board: Connect4,
	opponent: Opponent,
	link: ComponentLink<Self>,
	fetch_task: Option<FetchTask>,
}

pub enum Msg {
	MakeMove(usize),
	Reset,
	ChangeOpponent(Opponent),
	ReceiveResponse(Result<String, anyhow::Error>),
}

impl Connect4Page {
	fn update_score(&mut self, result: u8, human_flag: u8) {
		let ls = web_sys::window().unwrap().local_storage().unwrap().unwrap();
		let username = match ls.get_item("LoggedIn") {
			Ok(a) => match a {
				Some(b) => b,
				None => "".to_string(),
			},
			Err(_) => "".to_string(),
		};
		if username == "" {
			return;
		}
		let body = &json!({"username": &username, "mode": 0, "result": result, "human_flag": human_flag});
		let request = Request::post("http://localhost:8000/update_score")
			.header("Content-Type", "application/json")
			.body(Json(body))
			.expect("Build Request Failed");

		let callback =
			self.link
				.callback(|response: Response<Json<Result<String, anyhow::Error>>>| {
					let Json(data) = response.into_body();
					Msg::ReceiveResponse(data)
				});
		// fetch task for the req and callback
		let task = FetchService::fetch(request, callback).expect("failed to start request");
		// store the task
		self.fetch_task = Some(task);
	}
}

impl Component for Connect4Page {
	type Message = Msg;
	type Properties = ();
	fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
		Self {
			link,
			board: Connect4::initialize(),
			opponent: Opponent::ExpertMode,
			fetch_task: None,
		}
	}

	fn update(&mut self, msg: Self::Message) -> ShouldRender {
		let mut human_flag: u8 = 0; 
		match msg {
			Msg::MakeMove(col) => {
				let difficulty = match self.opponent {
					Opponent::Human => {
						human_flag = 1;
						0
					},
					Opponent::EasyMode => 1,
					Opponent::NormalMode => 2,
					Opponent::ExpertMode => 3,
				};
				if let Some(_) = self.board.winner {
					return false;
				} else if self.board.next_step == 42 {
					return false;
				}

				if self.board.place(col) == false {
					return false;
				}
				if let Some(winner) = self.board.winner {
					match winner {
						C4Piece::P1 => self.update_score(1, human_flag),
						C4Piece::P2 => self.update_score(0, human_flag),
					}
					return true;
				} else if self.board.winner.is_none() && self.board.termination {
					self.update_score(2, human_flag);
					return true;
				}
				if difficulty != 0 {
					self.board.place(con4_ai::AI_next_move(self.board, difficulty));
				}

				if let Some(winner) = self.board.winner {
					match winner {

						C4Piece::P1 => self.update_score(1, human_flag),
						C4Piece::P2 => self.update_score(0, human_flag),

					}
					return true;
				} else if self.board.winner.is_none() && self.board.termination {
					self.update_score(2, human_flag);
					return true;
				}
			}
			Msg::Reset => {
				self.board = Connect4::initialize();
			}
			Msg::ChangeOpponent(opponent) => {
				if self.board.next_step == 0 {
					self.opponent = opponent;
				}
			}
			Msg::ReceiveResponse(response) => match response.unwrap().as_str() {
				"Update success" => {}
				_ => {}
			},
		};

		true
	}

	fn change(&mut self, _props: Self::Properties) -> ShouldRender {
		true
	}

	fn view(&self) -> Html {

		let check_piece = move |row: usize, col: usize| -> Html {
			let mut classes = String::from("piece");

			if let Some(_) = self.board.winner {
				if self
					.board
					.check_win(self.board.current_player.switch())
					.unwrap()
					.contains(&[row, col])
				{
					classes.push_str(" piece--winner");
				}
			}

			classes.push_str(match self.board.board[row][col] {
				None => " piece--empty",
				Some(color) => match color {
					P1 => " piece--p1",
					P2 => " piece--p2",
				},
			});
			html! {<div class=classes></div>}
		};

		let game_status = move || -> Html {

			let status_text = match (self.board.termination,self.board.current_player, self.opponent, self.board.winner) {
				(false, P1, Opponent::Human, _) => "P1 MOVE",
				(false, P2, Opponent::Human, _) => "P2 MOVE",
				(false, P1,_,_) => "YOUR MOVE",
				(false, P2,_,_) => "YOUR MOVE",
				// who wins
				(true,_, Opponent::Human, Some(P1)) => "P1 WINS",
				(true,_, Opponent::Human, Some(P2)) => "P2 WINS",
				(true,_, _, Some(P1)) => "YOU WIN",
				(true, _,_, Some(P2)) => "COMPUTER WINS",
				(true,_,_,None) => "TIE GAME",
			};

			let text_color_class = move || -> &str {
				match (self.board.termination,self.board.current_player, self.board.winner) {
					(false,P1,_) => "game_status--p1",
					(false, P2,_) => "game_status--p2",
					// who wins
					(true,_,Some(P1)) => "game_status--p1",
					(true,_,_) => "game_status--p2",
				}
			};

			html! {
				<div class="game_status">
					<div class=format!("game_status--text-field {}", text_color_class())>{status_text}</div>
				</div>
			}
		};

		let opponent_buttons = move || -> Html {
			html! {
				<div class=format!("opponent {}", if self.board.next_step > 0 { "opponent--disabled" } else { "" }) >
					{
						Opponent::iter().map(|opponent| {
							html! {
								<button
									class=format!("opponent__button {}", if self.opponent == opponent {"opponent__button--selected"} else {""})
									onclick=self.link.callback(move |_| Msg::ChangeOpponent(opponent))
								>
									{opponent}
								</button>
						}}).collect::<Html>()
					}
				</div>
			}
		};

		let hover_col_class = move || -> &str {
			match self.board.current_player {
				P1 => "column--p1",
				P2 => "column--p2",
			}
		};

		let board_border_class = move || -> &str {
			match self.board.current_player {
				P1 => "board--p1",
				P2 => "board--p2",
			}
		};


		html! {
			<div class="container">
			<div class="selection">
				<button class="selection__reset" onclick=self.link.callback(move |_| Msg::Reset)>{"RESET"}</button>
				{opponent_buttons()}
			</div>
				<div class={format!("board {}", board_border_class())}>
				{
					(0..NUM_COLS).into_iter().map(|col| {
						return html! {
							<div class={format!("{}", hover_col_class())} onclick=self.link.callback(move |_| Msg::MakeMove(col))>
								{
									(0..NUM_ROWS).into_iter().map(|row| {
										return html! {
											<div class="cell">
												{check_piece(row, col)}
											</div>
										}
									}).collect::<Html>()
								}
							</div>
						}
					}).collect::<Html>()
				}
				</div>
				{game_status()}
				
			</div>
		}
	}
}
