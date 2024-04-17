use crate::{
	toot_and_otto::{
		toot_ai,
		toot_and_otto::{TootAndOtto, Player::*, NUM_COLS, NUM_ROWS, TOenum, TOenum::*},
	},
	types::opponent::Opponent,
};
use serde_json::json;
use strum::IntoEnumIterator;
use yew::format::Json;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct TootAndOttoPage {
	link: ComponentLink<Self>,
	board: TootAndOtto,
	vs: Opponent,
	fetch_task: Option<FetchTask>,
}

pub enum Msg {
	DropPiece(TOenum, usize),
	Reset,
	ChangeOpponent(Opponent),
	ReceiveResponse(Result<String, anyhow::Error>),
}

impl TootAndOttoPage {
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
		let body = &json!({"username": &username, "mode": 1, "result": result, "human_flag": human_flag});
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

impl Component for TootAndOttoPage {
	type Message = Msg;
	type Properties = ();
	fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
		Self {
			link,
			board: TootAndOtto::new(),
			vs: Opponent::Human,
			fetch_task: None,
		}
	}

	fn update(&mut self, msg: Self::Message) -> ShouldRender {
		let mut human_flag:u8 = 0;
		match msg {
			Msg::DropPiece(letter, col) => {
				let difficulty = match self.vs {
					Opponent::Human => {
						human_flag = 1;
						0
					},
					Opponent::EasyMode => 1,
					Opponent::NormalMode => 2,
					Opponent::ExpertMode => 3,
				};
				if self.board.termination == true {
					return false;
				} else if self.board.drop(letter, col) == false {
					return false;
				}

				if self.board.termination {
					match self.board.winner {
						None => {
							// TODO: Insert a tie into the db
							self.update_score(2, human_flag);
						}
						Some(winner) => match winner {
							OTTO => self.update_score(0, human_flag),
							TOOT => self.update_score(1, human_flag),
						},
					}
					return true;
				}

				if difficulty != 0 {
					let (best_col, best_letter) = toot_ai::AI_next_move(self.board, difficulty);
					self.board.drop(best_letter, best_col);
				}
				if self.board.termination {
					match self.board.winner {
						None => {
							// TODO: Insert a tie into the db
							self.update_score(2, human_flag);
						}
						Some(winner) => match winner {
							OTTO => self.update_score(0, human_flag),
							TOOT => self.update_score(1, human_flag),
						},
					}
					return true;
				}
			}
			Msg::ChangeOpponent(opponent) => {
				if self.board.next_step == 0 {
					self.vs = opponent;
				}
			}
			Msg::Reset => {
				self.board = TootAndOtto::new();
			}
			Msg::ReceiveResponse(response) => match response.unwrap().as_str() {
				"Update success" => {}
				_ => {}
			},
		}

		true
	}

	fn change(&mut self, _props: Self::Properties) -> ShouldRender {
		false
	}

	fn view(&self) -> Html {
		let check_piece = move |row: usize, col: usize| -> Html {
			let mut classes = String::from("piece");

			match self.board.check_win(TOOT) {
				None => {}
				Some(coordinates) => {
					if coordinates.contains(&[row, col]) {
						classes.push_str(" piece--winner");
					}
				}
			}

			match self.board.check_win(OTTO) {
				None => {}
				Some(coordinates) => {
					if coordinates.contains(&[row, col]) {
						classes.push_str(" piece--winner");
					}
				}
			}

			classes.push_str(match self.board.board[row][col] {
				None => " piece--empty",
				Some(letter) => match letter {
					T => " piece--toot-n-otto",
					O => " piece--toot-n-otto",
				},
			});

			let letter = match self.board.board[row][col] {
				None => String::from(""),
				Some(l) => format!("{}", l),
			};

			html! {<div class=classes>{letter}</div>}
		};

		let game_status = move || -> Html {

			let arrow_text = match (self.board.termination,self.board.current_player, self.vs, self.board.winner) {
				(false, TOOT, Opponent::Human,_) => "TOOT MOVE",
				(false, OTTO, Opponent::Human,_) => "OTTO MOVE",
				(false, TOOT,_,_) => "YOUR(TOOT) MOVE",
				(false, OTTO,_,_) => "YOUR(OTTO) MOVE",
				// who wins
				(true, _, Opponent::Human, Some(TOOT)) => "TOOT WINS",
				(true, _, Opponent::Human, Some(OTTO)) => "OTTO WINS",
				(true, _, _, Some(TOOT)) => "YOU(TOOT) WIN",
				(true, _,_ , Some(OTTO)) => "COMPUTER(OTTO) WINS",
				(true, _, _, None) => "TIE GAME",
			};

			let text_color_class = move || -> &str {
				match (self.board.termination,self.board.current_player, self.board.winner) {
					(false,TOOT, _) => "game_status--p1",
					(false, OTTO, _) => "game_status--p2",
					// who wins
					(true, _, Some(TOOT)) => "game_status--p1",
					(true, _, _) => "game_status--p2",

				}
			};

			html! {
				<div class="game_status">
					<div class=format!("game_status--text-field {}", text_color_class())>{arrow_text}</div>
				</div>
			}
		};

		let opponent_buttons = move || -> Html {
			html! {
				Opponent::iter().map(|opponent| {
					html! {
						<button
							class=format!("opponent__button {}", if self.vs == opponent {"opponent__button--selected"} else {""})
							onclick=self.link.callback(move |_| Msg::ChangeOpponent(opponent))
						>
							{opponent}
						</button>
					}
				}).collect::<Html>()
			}
		};

		let floating_pieces = move |col: usize| -> Html {
			let floating_piece_color =  move || -> &str {
				match self.board.current_player {
					TOOT => "piece--floating--p1",
					OTTO => "piece--floating--p2",
				}
			};

			let show_piece = move |letter| -> &str {
				let player_index = match self.board.current_player {
					TOOT => 0,
					OTTO => 1,
				};

				let letter_index = match letter {
					T => 0,
					O => 1,
				};

				if self.board.countings[player_index][letter_index] == 0 {
					return "piece--floating--hidden";
				} else {
					return "";
				}
			};

			html! {
				TOenum::iter().map(|letter| html! {
					<div class="cell cell--floating">
							<div
								class=format!("piece piece--floating {} {}", floating_piece_color(), show_piece(letter))
								onclick=self.link.callback(move |_| Msg::DropPiece(letter, col))
							>
								{letter}
							</div>
						</div>
				}).collect::<Html>()
			}
		};

		let board_border_class = move || -> &str {
			match self.board.current_player {
				TOOT => "board--p1",
				OTTO => "board--p2",
			}
		};

		html! {
			<div class="container">
				<div class="selection">
					<button class="selection__reset" onclick=self.link.callback(move |_| Msg::Reset)>{"RESET"}</button>
					<div class=format!("opponent {}", if self.board.next_step > 0 { "opponent--disabled" } else { "" })>
						{opponent_buttons()}
					</div>
				</div>
				<div class={format!("board {}", board_border_class())}>
					{
						(0..NUM_COLS).into_iter().map(|col| {
							return html! {
								<div class="column">
									{ floating_pieces(col) }
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
				<div class="piece-counts__container">
					<div class="piece-counts__p1">
						<p id="left-info">{format!("TOOT - T's: {} O's: {}", self.board.countings[0][0], self.board.countings[0][1])}</p>
					</div>
					<div class="piece-counts__p2">
						<p id="left-info">{format!("OTTO - T's: {} O's: {}", self.board.countings[1][0], self.board.countings[1][1])}</p>
					</div>
				</div>
				
			</div>
		}
	}
}
