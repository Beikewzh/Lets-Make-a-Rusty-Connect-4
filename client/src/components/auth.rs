// Login, Reg, & Logout
use serde_json::json;
use yew::format::Json;
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

// Struct for Auth Page
pub struct AuthPage {
    link: ComponentLink<Self>,
    username: String,
    error: Option<String>,
    password: String,
    fetch_task: Option<FetchTask>,
}

// Message passing for Login:
pub enum Message {
    Login(bool),
    Logout,
    UpdateUsername(String),
    UpdatePassword(String),
    ReceiveResponse(Result<String, anyhow::Error>),
}

impl AuthPage {
    // Send login request to the server and establish a call back
    pub fn login(&mut self, login: bool) {
        if self.username == "" || self.password == "" {
            return;
        }
        let body = &json!({"username": &self.username, "password": &self.password});
        let request = Request::post(format!(
            "http://localhost:8000/{}",
            if login { "login" } else { "new_user" }
        ))
            .header("Content-Type", "application/json")
            .body(Json(body))
            .expect("Build Request Failed");
        // Creat a callback that will receive a turple of data:
        let callback = self
            .link
            .callback(|response: Response<Json<Result<String, anyhow::Error>>>| {
                let Json(data) = response.into_body();
                Message::ReceiveResponse(data)
            });
        // Fetch service for the req and callback
        let task = FetchService::fetch(request, callback).expect("failed to start request");
        // Store the task prevent losting immediately
        self.fetch_task = Some(task);
    }
}

impl Component for AuthPage {
    type Message = Message;
    type Properties = ();
    // Create component
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            error: None,
            username: "".to_string(),
            password: "".to_string(),
            fetch_task: None,
        }
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        match message {
            Message::Login(login) => {
                if self.username.len() == 0 || self.password.len() == 0 {
                    return false;
                }

                if self.username.contains(" ") {
                    self.error = Some(String::from("Invalid username"));
                    return true;
                }

                // Send request to server
                self.login(login);
            }
            Message::UpdateUsername(username) => {
                self.username = username;
                self.error = None;
            }
            Message::UpdatePassword(password) => {
                self.password = password;
                self.error = None;
            }
            Message::ReceiveResponse(response) => {
                // Parse response from server
                let window = web_sys::window().unwrap();
                let loc_storage = window.local_storage().unwrap().unwrap();
                match response.unwrap().as_str() {
                    "Login OK" => {
                        // Add logged in user to local storage
                        loc_storage.set_item("LoggedIn", &self.username)
                            .expect("Error setting user login");

                        // Navigate to connect 4 page
                        let document = window.document().unwrap();
                        let location = document.location().unwrap();
                        let url = format!("{}//{}/{}", location.protocol().expect("error"),
                                          location.host().expect("error"), "connect-4/");
                        location.set_href(&url).expect("failed");
                    }
                    "Created user" => {
                        // Add logged in user to local storage
                        loc_storage.set_item("LoggedIn", &self.username)
                            .expect("Error setting user login");

                        // Navigate to connect 4 page
                        let document = window.document().unwrap();
                        let location = document.location().unwrap();
                        let url = format!(
                            "{}//{}/{}",
                            location.protocol().expect("error"),
                            location.host().expect("error"),
                            "connect-4/"
                        );
                        location.set_href(&url).expect("failed");
                    }
                    "Login Failed" => self.error = Some(String::from("Login Failed")),
                    "Username Taken" => self.error = Some(String::from("This username is already taken.")),
                    _ => {
                        // Clear user login credential input:
                        loc_storage.set_item("LoggedIn", &"").expect("Clear Failed");
                    }
                }
            }
            Message::Logout => {
                // Logout and clear local storage
                let window = web_sys::window().unwrap();
                let loc_storage = window.local_storage().unwrap().unwrap();
                loc_storage.set_item("LoggedIn", &"")
                    .expect("Clear Failed");
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        // Get the web page view for Auth page:
        let window = web_sys::window().unwrap();
        let loc_storage = window.local_storage().unwrap().unwrap();
        let username = match loc_storage.get_item("LoggedIn") {
            Ok(item) => match item {
                Some(name) => name,
                None => "".to_string(),
            },
            Err(_) => "".to_string(),
        };

        let submit_button_class =
            if self.username.len() > 0 && self.password.len() > 0 {
                ""
            } else {
                "button--disabled"
            };

 let login_status = move || -> Html {
			match username.as_str() {
				"" => html! {
		    <div class = "login_box">
			<h2 class = "prompt1">{"Play with friends or Combat the AI in the"}</h2>
			<h2 class = "prompt2">{"Legendary Board Game System"}</h2>
			<div class = "auth__container auth__container--username">
			<label class = {if self.username.len() > 0 {"raised"} else {""}}>
			    {"Username"}
			</label>
			    <input type = "text" name = "username" id = "login-username" value = { &self.username }
				oninput = self.link.callback(|e: InputData| Message::UpdateUsername(e.value))/>
			</div>
			<div class = "auth__container auth__container--password">
			    <label class = {if self.password.len() > 0 {"raised"} else {""}}>{"Password"}</label>
			    <input type = "password" name="password" id="login-password"
				    oninput=self.link.callback(|e: InputData| Message::UpdatePassword(e.value))/>
			</div>
			{
			    html! {
				    if let Some(message) = &self.error {
				        html! {<p class = "auth__error">{message}</p>}
				    } else {
				        html! {}
                    }
			    }
			}
			<button class = submit_button_class id = "button--login"
				onclick = self.link.callback(move |_| Message::Login(true))>{"Login"}
			</button>
			<div class = "separator__container">
			    <div class = "separator__line"></div>
			    <p class = "separator__label">{"OR"}</p>
			    <div class = "separator__line"></div>
			</div>
			<button class = submit_button_class id="button-reg"
			    onclick = self.link.callback(move |_| Message::Login(false))>{"Sign up"}</button>
		    </div>
		},
				_ => html! {
		    <div class = "wrapper row">
			    <h2 class = "prompt2">{format!("Welcome,\u{00a0}{}! Hope You Love These Games!", username)}</h2>

			    <div class = "tooltip col-lg-6 col-md-12">
			        <p style = "color:black">{"CONNECT\u{00a0}FOUR"}</p>
			        <span class = "tooltiptext" id = "login">{"Object: Connect four of your checkers in a row while
			        preventing your opponent from doing the same. But, look out – your opponent can sneak
			        up on you and win the game! —Milton Bradley, Connect Four \"Pretty Sneaky, Sis\" \
			        television commercial, 1977"}</span>
			    </div>

			    <div class = "tooltip col-lg-6 col-md-12">
			        <p style = "color:black">{"TOOT\u{00a0}&\u{00a0}OTTO"}</p>
			        <span class = "tooltiptext" id = "login">{"One player is TOOT and the other player is OTTO. Each takes
			        six O's and six T's. The first player who spells his or her name - up, down, sideways,
			        or on the diagonal - wins!\"Simply drop all \
			        the T's and O's in the tower and when it's full, search for how many TOOT's \
			        and OTTO's you can discover!\""}</span>
			    </div>
			    <button id = "button-logout" onclick = self.link.callback(move |_| Message::Logout)>{"Logout"}</button>
		    </div>
		},
			}
		};

		html! {
	        <div class = "login-page">{login_status()}</div>
	    }
	}
}
