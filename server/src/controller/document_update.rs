/*
Database calls to update scores and get scores
*/
use std::ptr::null;
use crate::MongoDB;
use mongodb::bson::{doc, Document, from_document};

// Game stats struct
#[derive(Debug, Serialize, Deserialize, FromForm)]
pub struct GameStats {
    pub username: String,
    pub c_rank: i32,
    pub t_rank: i32,
    pub c_score: i32,
    pub t_score: i32,
    pub c_win_count: i32,
    pub c_lose_count: i32,
    pub c_tie_count: i32,
    pub t_win_count: i32,
    pub t_lose_count: i32,
    pub t_tie_count: i32,
}

// Champ stats struct
#[derive(Debug, Serialize, Deserialize, FromForm)]
pub struct ChampStats {
    pub username: String,
    pub score: i32,
    pub win_count: i32,
    pub lose_count: i32,
    pub tie_count: i32,
}

// Update score struct
#[derive(Debug, Serialize, Deserialize, FromForm)]
pub struct ScoreUpdate {
    pub username: String,
    pub mode: u8, // Mode 0 connect 4, 1 toot
    pub result: u8,  // Result 0 lost, 1 win, 2 tie
    pub human_flag: u8,
}

impl MongoDB {

    // DB function to update the score
    pub fn update_score(&mut self, username: &String, mode: u8, result: u8, human_flag: u8) -> Result<bool, mongodb::error::Error> {
        let score_document = self.db.collection::<Document>("scores");
        if human_flag == 1 {
            return Ok(true);
        }
        let score = match mode {
            0 => {
                if result == 1 {
                    doc! { "c_win_count": 1, "c_score": 5, "c_rank": -5, }
                } else if result == 0 {
                    doc! { "c_lose_count": 1, "c_score": -3, "c_rank": -1, }
                } else {
                    doc! { "c_tie_count": 1, "c_score": 2, "c_rank": -2, }
                }
            }
            1 => {
                if result == 1 {
                    doc! { "t_win_count": 1, "t_score": 5, "t_rank": -5, }
                } else if result == 0 {
                    doc! { "t_lose_count": 1, "t_score": -3, "t_rank": -1, }
                } else {
                    doc! { "t_tie_count": 1, "t_score": 2, "t_rank": -2, }
                }
            }
            _ => return Ok(false),
        };
        score_document.update_one(
            doc! {
				"username": username
			},
            doc! {"$inc": score},
            None,
        )?;
        Ok(true)
    }

    // DB function to get the game scores and champion information:
    pub fn get_game_score(&mut self, username: String)
        -> Result<Option<(GameStats, ChampStats, ChampStats)>, mongodb::error::Error> {
        let score_document = self.db.collection::<Document>("scores");
        let res = score_document.find_one(doc! {"username": &username}, None)?;
        return match res {
            Some(document) => {
                let mut c_rank_value = document.get("c_rank").unwrap().as_i32().unwrap_or(-255);
                let mut t_rank_value = document.get("t_rank").unwrap().as_i32().unwrap_or(-255);
                if c_rank_value <= 0 || c_rank_value > 100000 {
                    c_rank_value = 1;
                    let set_rank = doc!{ "c_rank": 1 };
                    score_document.update_one(
                        doc! {
                            "username": username.clone()
			            },
                        doc! {"$set": set_rank}, None,)?;
                }
                if t_rank_value <= 0 || t_rank_value > 100000 {
                    t_rank_value = 1;
                    let set_rank = doc!{ "t_rank": 1 };
                    score_document.update_one(
                        doc! {
                            "username": username.clone()
			            },
                        doc! {"$set": set_rank}, None,)?;
                }
                let game_info = GameStats {
                    username: username.to_string(),
                    c_rank: c_rank_value,
                    t_rank: t_rank_value,
                    c_score: document.get("c_score").unwrap().as_i32().unwrap_or(0),
                    t_score: document.get("t_score").unwrap().as_i32().unwrap_or(0),
                    c_win_count: document.get("c_win_count").unwrap().as_i32().unwrap_or(0),
                    c_lose_count: document.get("c_lose_count").unwrap().as_i32().unwrap_or(0),
                    c_tie_count: document.get("c_tie_count").unwrap().as_i32().unwrap_or(0),
                    t_win_count: document.get("t_win_count").unwrap().as_i32().unwrap_or(0),
                    t_lose_count: document.get("t_lose_count").unwrap().as_i32().unwrap_or(0),
                    t_tie_count: document.get("t_tie_count").unwrap().as_i32().unwrap_or(0),
                };

                // Todo: Call other function or just query here: players with highest scores:
                // find one good use here i hate cursor pipelines.
                let champion_c4 = score_document.aggregate(vec![doc!{"$sort": {"c_score": -1}},  doc!{"$limit": 1}], None).unwrap().next().unwrap();
                let champion_ot = score_document.aggregate(vec![doc!{"$sort": {"t_score": -1}},  doc!{"$limit": 1}], None).unwrap().next().unwrap();

                // let champion_c4 = score_document.find_one(doc!{"username": "9".to_string()}, None)?;
                // let champion_ot = score_document.find_one(doc!{"username": "9".to_string()}, None)?;
                let c4_champ:ChampStats;
                let ot_champ:ChampStats;
                match champion_c4 {
                    Err(_) => {
                        c4_champ = ChampStats {
                            username: "N/A".to_string(),
                            score: 0,
                            win_count: 0,
                            lose_count: 0,
                            tie_count: 0
                        };
                    }
                    Ok(document) => {
                        c4_champ = ChampStats {
                            username: document.get("username").unwrap().to_string(),
                            score: document.get("c_score").unwrap().as_i32().unwrap_or(0),
                            win_count: document.get("c_win_count").unwrap().as_i32().unwrap_or(0),
                            lose_count: document.get("c_lose_count").unwrap().as_i32().unwrap_or(0),
                            tie_count: document.get("c_tie_count").unwrap().as_i32().unwrap_or(0),
                        };
                    }
                }
                match champion_ot {
                    Err(_) => {
                        ot_champ = ChampStats {
                            username: "N/A".to_string(),
                            score: 0,
                            win_count: 0,
                            lose_count: 0,
                            tie_count: 0
                        };
                    }
                    Ok(document) => {
                        ot_champ = ChampStats {
                            username: document.get("username").unwrap().to_string(),
                            score: document.get("t_score").unwrap().as_i32().unwrap_or(0),
                            win_count: document.get("t_win_count").unwrap().as_i32().unwrap_or(0),
                            lose_count: document.get("t_lose_count").unwrap().as_i32().unwrap_or(0),
                            tie_count: document.get("t_tie_count").unwrap().as_i32().unwrap_or(0),
                        }
                    }
                }

                Ok(Some((game_info, c4_champ, ot_champ)))
            }
            None => Ok(None),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, FromForm)]
pub struct User {
    pub username: String,
    pub password: String,
}

impl MongoDB {
    // Add a new user to the database:
    pub fn add_user(&mut self, username: &String, password: &String) -> Result<(), mongodb::error::Error> {
        let user_db = self.db.collection::<Document>("users");
        let user = doc! {"username" : username, "password": password};
        user_db.insert_one(user, None)?;
        let score_document = self.db.collection("scores");
        let stats = doc! {
			"username": username,
            "c_rank": 100000,
            "t_rank": 100000,
            "c_score": 0,
            "t_score": 0,
			"c_win_count": 0,
			"c_lose_count": 0,
			"c_tie_count": 0,
			"t_win_count": 0,
			"t_lose_count": 0,
			"t_tie_count": 0
		};
        score_document.insert_one(stats, None)?;
        Ok(())
    }

    // Check if user exists in the database and log in if does:
    pub fn authentication_verify(&mut self, username: &String, password: &String) -> Result<bool, mongodb::error::Error> {
        let user_db = self.db.collection::<Document>("users");
        let user = doc! {"username" : username, "password": password};
        let res = user_db.find_one(user, None)?;
        return match res {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }
}