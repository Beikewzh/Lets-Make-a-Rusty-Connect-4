use mongodb::error::Error;
use crate::MongoDB;
use rocket::http::RawStr;
use rocket_contrib::json::Json;
use crate::document_update::{ChampStats, GameStats, ScoreUpdate, User};

// Post request to update game scores
#[post("/update_score", format = "application/json", data = "<score>")]
pub fn update_score(score: Json<ScoreUpdate>) -> Json<String> {
    return match MongoDB::new() {
        // Establish connection
        Ok(mut db) => {
            match db.update_score(&score.username, score.mode, score.result, score.human_flag) {
                // Update score
                Ok(res) => {
                    if res {
                        Json(String::from("Update success"))
                    } else {
                        Json(String::from("Update failed"))
                    }
                }
                Err(_) => Json(String::from("Failure:")),
            }
        },
        Err(_) => Json(String::from("Failure: Bad Connection")),
    }
}

// Get request to obtain user stats
#[get("/scores/<username>")]
pub fn get_scores(username: &RawStr) -> Json<(GameStats, ChampStats, ChampStats)> {
    let err = GameStats {
        // error struct. Sends this if no user found
        username: "".to_string(),
        c_rank: 0,
        t_rank: 0,
        c_score: 0,
        t_score: 0,
        c_win_count: 0,
        c_lose_count: 0,
        c_tie_count: 0,
        t_win_count: 0,
        t_lose_count: 0,
        t_tie_count: 0,
    };
    let err_champ_c4 = ChampStats {
        username: "".to_string(),
        score: 0,
        win_count: 0,
        lose_count: 0,
        tie_count: 0
    };
    let err_champ_ot = ChampStats {
        username: "".to_string(),
        score: 0,
        win_count: 0,
        lose_count: 0,
        tie_count: 0
    };
    return match MongoDB::new() {
        // Mongodb Connection
        Ok(mut db) => {
            match db.get_game_score(username.to_string()) {
                // Gets the user's game statistics
                Ok(stats) => {
                    if stats.is_none() {
                        Json((err, err_champ_c4, err_champ_ot))
                    } else {
                        Json(stats.unwrap())
                    }
                }
                Err(_) => Json((err, err_champ_c4, err_champ_ot)),
            }
        }
        Err(_) => Json((err, err_champ_c4, err_champ_ot)),
    }
}

// Post request to create a new user
#[post("/new_user", format = "application/json", data = "<user>")]
pub fn new_user(user: Json<User>) -> Json<String> {
    match MongoDB::new() {
        // Establish connection
        Ok(mut db) => {
            match db.add_user(&user.username, &user.password) {
                // Add user to db
                Ok(()) => {}
                Err(_) => return Json(String::from("Username Taken")),
            };
        }
        Err(_) => return Json(String::from("Database Connection Failed")),
    }
    Json(String::from(format!("Created user")))
}

// Post request to verify login request for existing user candidate:
#[post("/login", format = "application/json", data = "<user>")]
pub fn authentication_verify(user: Json<User>) -> Json<String> {
    match MongoDB::new() {
        // Establish connection
        Ok(mut db) => match db.authentication_verify(&user.username, &user.password) {
            Ok(res) => {
                if res == false {
                    return Json(String::from("Login Failed"));
                }
            }
            Err(_) => return Json(String::from("Login Failed")),
        },
        Err(_) => return Json(String::from("Database Connection Failed")),
    }
    Json(String::from(format!("Login OK")))
}
