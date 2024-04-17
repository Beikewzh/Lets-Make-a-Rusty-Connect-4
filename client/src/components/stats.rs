// Game Statistics
use serde::{Deserialize, Serialize};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::{prelude::*, format::{Json, Nothing}};
use crate::switch::{AppRoute, RouterButton};

// Struct for Stats Page:
pub struct Stats {
    component_link: ComponentLink<Self>,
    username: String,
    fetch_task: Option<FetchTask>,
    game_info: Option<GameStats>,
    champion_c4: Option<ChampStats>,
    champion_ot: Option<ChampStats>,
    init: bool,
}

// Message passing for stats:
pub enum Message {
    ReceiveResponse(Result<(GameStats, ChampStats, ChampStats), anyhow::Error>),
}

// Game info struct
#[derive(Debug, Serialize, Deserialize)]
pub struct GameStats {
    pub username: String,
    pub c_rank: i32,
    pub t_rank: i32,
    pub c_score: i32,
    pub t_score: i32,
    pub c_win_count: i32,
    // Connect 4 Total wins
    pub c_lose_count: i32,
    // Connect 4 Total loses
    pub c_tie_count: i32,
    // Connect 4 Total ties
    pub t_win_count: i32,
    // toot and otto Total wins
    pub t_lose_count: i32,
    // toot and otto Toal loses
    pub t_tie_count: i32, // toot and otto Total ties
}

// Champ stats struct
#[derive(Debug, Serialize, Deserialize)]
pub struct ChampStats {
    pub username: String,
    pub score: i32,
    pub win_count: i32,
    pub lose_count: i32,
    pub tie_count: i32,
}

impl Stats {
    // Request to server to fetch stats
    fn get_stats(&mut self, user: String) {
        if user == "" {
            return;
        }
        log::info!("User {}", user);
        let request = Request::get(format!("http://localhost:8000/scores/{}", &user))
            .header("Content-Type", "application/json")
            .body(Nothing)
            .expect("Build Request Failed");
        // Creat a callback that will receive a turple of data:
        let callback = self.component_link.callback(
            |response: Response<Json<Result<(GameStats, ChampStats, ChampStats), anyhow::Error>>>| {
                let Json(data) = response.into_body();
                Message::ReceiveResponse(data)
            },
        );
        // Fetch service for the req and callback
        let task = FetchService::fetch(request, callback).expect("failed to start request");
        // Store the task prevent losting immediately
        self.fetch_task = Some(task);
    }
}

impl Component for Stats {
    type Message = Message;
    type Properties = ();
    // Create stats component
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let loc_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
        let username = match loc_storage.get_item("LoggedIn") {
            Ok(item) => match item {
                Some(name) => name,
                None => "".to_string(),
            },
            Err(_) => "".to_string(),
        };
        Self {
            component_link: link,
            username: username,
            fetch_task: None,
            champion_c4: None,
            champion_ot: None,
            game_info: None,
            init: true,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        // Get stats and champion info on first rendering
        if first_render && self.username != "" {
            let user = self.username.to_string();
            self.get_stats(user);
        }
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        // Update: Match message and parse data
        if self.init {
            let user = self.username.to_string();
            self.get_stats(user);
            self.init = false;
        }
        match message {
            Message::ReceiveResponse(response) => {
                match response {
                    Ok(res) => {
                        self.game_info = Some(res.0);
                        self.champion_c4 = Some(res.1);
                        self.champion_ot = Some(res.2);
                    }
                    Err(_) => {
                        self.game_info = None;
                        self.champion_ot = None;
                        self.champion_c4 = None;
                    }
                }
            }
        }
        true
    }

    fn change(&mut self, _propertiess: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        // Obtain the Webpage View:
        let mut stat_results = html! {};
        let loc_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
        let username = match loc_storage.get_item("LoggedIn") {
            Ok(item) => match item {
                Some(name) => name,
                None => "".to_string(),
            },
            Err(_) => "".to_string(),
        };
        if self.game_info.as_ref().is_some() && self.champion_ot.is_some() && self.champion_c4.is_some() {
            let game_stats = self.game_info.as_ref().unwrap();
            let champ_c4 = self.champion_c4.as_ref().unwrap();
            let champ_ot = self.champion_ot.as_ref().unwrap();
            let ot_champ_name = champ_ot.username.replace("\"", "");
            let c4_champ_name = champ_c4.username.replace("\"", "");

            if game_stats.username == "" {
                stat_results = html! {
                    <h1 class="stats_status">{"Statistics Not Available"}</h1>
                };
            } else {
                stat_results = html! {
                    <div class="wrapper row">
                        <p class="info col-md-12">{"Statistics\u{00a0}&\u{00a0}Champions"}</p>
                        <section class="col-lg-6 col-md-12">
                        <div class="stats">
                            <div class="title">{format!("{}'S\u{00a0}PLAYER\u{00a0}STATS", game_stats.username)}
                            <span class="tooltip stats-icon">{"\u{24d8}"}<pre class="tooltiptext" id="rule">
                            {"Score Rule:\nWin +5 Lose -3 Tie +2\n(Playing with Friends Won't Affect Your Score!!)\n\nExperience Based Ranking Rule:\nThe MORE you play, the HIGHER you're ranked!!"}
                        </pre></span></div>
                    <h3 class="stats_entry">{format!("CONNECT 4")}</h3>
                    <div class="pairs">
                    <div class="tooltip-pair pair-4">
                        <span class="tooltiptext">{format!("Experience Ranked TOP {}", game_stats.c_rank)}</span>
                        <h2 class="pair-content">{format!("Score")}</h2>
                        <h2 class="pair-content">{format!("{}", game_stats.c_score)}</h2>
                    </div>
                    <div class="pair-4">
                        <h2 class="pair-content">{format!("Win")}</h2>
                        <h2 class="pair-content">{format!("{}", game_stats.c_win_count)}</h2>
                    </div>
                    <div class="pair-4">
                        <h2 class="pair-content">{format!("Lose")}</h2>
                        <h2 class="pair-content">{format!("{}", game_stats.c_lose_count)}</h2>
                    </div>
                    <div class="pair-4">
                        <h2 class="pair-content">{format!("Tie")}</h2>
                        <h2 class="pair-content">{format!("{}", game_stats.c_tie_count)}</h2>
                    </div>
                </div>
                    <h3 class="stats_entry">{format!("TOOT & OTTO")}</h3>
                    <div class="pairs">
                        <div class="pair-4 tooltip-pair">
                        <span class="tooltiptext">{format!("Experience Ranked TOP {}", game_stats.t_rank)}</span>
                        <h2 class="pair-content">{format!("Score")}</h2>
                        <h2 class="pair-content">{format!("{}", game_stats.t_score)}</h2>
                    </div>
                        <div class="pair-4">
                        <h2 class="pair-content">{format!("Win")}</h2>
                            <h2 class="pair-content">{format!("{}", game_stats.t_win_count)}</h2>
                        </div>
                        <div class="pair-4">
                            <h2 class="pair-content">{format!("Lose")}</h2>
                            <h2 class="pair-content">{format!("{}", game_stats.t_lose_count)}</h2>
                        </div>
                        <div class="pair-4">
                            <h2 class="pair-content">{format!("Tie")}</h2>
                            <h2 class="pair-content">{format!("{}", game_stats.t_tie_count)}</h2>
                        </div>
                    </div>
                </div>
            </section>

            <section class="col-lg-6 col-md-12">
                <div class="stats">
                    <div class="title">{format!("WORLD\u{00a0}LEADERBOARD")}</div>
                        <div class="pairs">
                            <h3 class="stats_entry">{format!("{}, CONNECT\u{00a0}4\u{00a0}CHAMPION", c4_champ_name)}</h3>
                            <div class="pair-4">
                                <h2 class="pair-content">{format!("Score")}</h2>
                                <h2 class="pair-content">{format!("{}", champ_c4.score)}</h2>
                            </div>
                            <div class="pair-4">
                                <h2 class="pair-content">{format!("Win")}</h2>
                                <h2 class="pair-content">{format!("{}", champ_c4.win_count)}</h2>
                            </div>
                            <div class="pair-4">
                                <h2 class="pair-content">{format!("Lose")}</h2>
                                <h2 class="pair-content">{format!("{}", champ_c4.lose_count)}</h2>
                            </div>
                            <div class="pair-4">
                                <h2 class="pair-content">{format!("Tie")}</h2>
                                <h2 class="pair-content">{format!("{}", champ_c4.tie_count)}</h2>
                            </div>
                        </div>
                        <h3 class="stats_entry">{format!("{}, TOOT\u{00a0}&\u{00a0}OTTO\u{00a0}CHAMPION", ot_champ_name)}</h3>
                        <div class="pairs">
                            <div class="pair-4">
                                <h2 class="pair-content">{format!("Score")}</h2>
                                <h2 class="pair-content">{format!("{}",champ_ot.score)}</h2>
                            </div>
                            <div class="pair-4">
                                <h2 class="pair-content">{format!("Win")}</h2>
                                <h2 class="pair-content">{format!("{}", champ_ot.win_count)}</h2>
                            </div>
                            <div class="pair-4">
                                <h2 class="pair-content">{format!("Lose")}</h2>
                                <h2 class="pair-content">{format!("{}", champ_ot.lose_count)}</h2>
                            </div>
                            <div class="pair-4">
                                <h2 class="pair-content">{format!("Tie")}</h2>
                                <h2 class="pair-content">{format!("{}", champ_ot.tie_count)}</h2>
                            </div>
                        </div>
                    </div>
                </section>
            </div>
        }
            }
        } else if username == "" {
            stat_results = html! {
                <div class = "wrapper row">
                    <h2 class = "Brand">{"L.B.G.S."}</h2>
                    <h2 class = "login-prompt">{"Login Required for Game Statistics"}</h2>
                    <div class = "col-md-12 button-div"><RouterButton route=AppRoute::Login>{"Login"}</RouterButton></div>

                    <div class="tooltip col-lg-6 col-md-12">
                        <p>{"CONNECT\u{00a0}FOUR"}</p>
                        <span class="tooltiptext">{"Object: Connect four of your checkers in a row while
                        preventing your opponent from doing the same. But, look out – your opponent can sneak
                        up on you and win the game! —Milton Bradley, Connect Four \"Pretty Sneaky, Sis\" \
                        television commercial, 1977"}</span>
                    </div>

                    <div class="tooltip col-lg-6 col-md-12">
                        <p>{"TOOT\u{00a0}&\u{00a0}OTTO"}</p>
                        <span class="tooltiptext">{"One player is TOOT and the other player is OTTO. Each takes
                        six O's and six T's. The first player who spells his or her name - up, down, sideways,
                        or on the diagonal - wins!\"Simply drop all \
                        the T's and O's in the tower and when it's full, search for how many TOOT's \
                        and OTTO's you can discover!\""}</span>
                    </div>
                </div>
            };
        }
        html! {
            <div class="stats_page bg">{stat_results}</div>
        }
    }
}
