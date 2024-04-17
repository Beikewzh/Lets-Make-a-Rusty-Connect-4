use std::io;
use strum_macros::EnumIter;

#[path = "../../src/connect4/connect4.rs"]
mod connect4;
use connect4::{Connect4, C4Piece};

#[path = "../../src/connect4/con4_ai.rs"]
mod con4_ai;

#[path = "../../src/toot_and_otto/toot_and_otto.rs"]
mod toot_and_otto;
use toot_and_otto::{TootAndOtto, Player, TOenum};

#[path = "../../src/toot_and_otto/toot_ai.rs"]
mod toot_ai;

fn std_catch_C4() -> (usize, bool) 
{
    let mut input = String::new();

    if let Err(_) = io::stdin().read_line(&mut input) 
    {
        println!("Invalid input, please try again!");
        return (0, false);
    };

    let input: usize = match input.trim().parse() 
    {
        Err(_) => 
        {
            println!("Invalid input, please try again!");
            return (0, false);
        }
        Ok(input) => input,
    };

    (input, true)
}

fn std_catch_OT() -> (char, usize, bool) 
{
    let mut input = String::new();

    if let Err(_) = io::stdin().read_line(&mut input) 
    {
        println!("Invalid input, please try again!");
        return ('X', 0, false);
    };

    let split_input: Vec<&str> = input.trim().split_whitespace().collect();

    let letter: char = match split_input[0].parse() 
    {
        Err(_) => {
            println!("Invalid input, please try again!");
            return ('X', 0, false);
        }
        Ok(letter) => letter,
    };

    let col: usize = match split_input[1].parse() 
    {
        Err(_) => {
            println!("Invalid Input, Try Again");
            return ('X', 0, false);
        }
        Ok(col) => col,
    };

    (letter, col, true)
}

fn game_connect4(oppo : &str) {
    let mut oppo_choice;

    match oppo 
    {     
        "1" => {
            oppo_choice = 0;
        },
        "2" => {
            oppo_choice = 1;
        },
        "3" => {
            oppo_choice = 2;
        },
        "4" => {
            oppo_choice = 3;
        },
        _ => {
            println!("Invalid Opponent Choice. Setting to AI - EASY");
            oppo_choice = 1;
        }
    }

    let mut connect4 = Connect4::initialize();
    let mut player = C4Piece::P2;

    println!("{}", connect4);

    loop 
    {
        let winner = match connect4.check_win(player) 
        {
            Some(_) => Some(player),
            None => None,
        };

        if winner.is_some() 
        {
            break;
        }

        player = player.switch();

        println!("--> {} Player Turn <--", player);
        println!("Enter <0-1-2-3-4-5-6>");

        if (player == C4Piece::P1) || (oppo_choice == 0) 
        {
            let mut column: usize;
            let mut validation = false;

            while !validation 
            {
                let result = std_catch_C4();
                column = result.0;
                validation = result.1;

                if !validation 
                {
                    continue;
                }
                validation = connect4.place(column);
            }
        }
        else 
        {
            connect4.place(con4_ai::AI_next_move(connect4.clone(), oppo_choice));
        }

        println!("{}", connect4);
    }
    
    println!("----------------------------------------------------");
    println!("{} Player Won!", player);
}

fn game_TOTO(oppo : &str) {

    let mut oppo_choice;

    match oppo 
    {
        "1" => {
            oppo_choice = 0;
        },
        "2" => {
            oppo_choice = 1;
        },
        "3" => {
            oppo_choice = 2;
        },
        "4" => {
            oppo_choice = 3;
        },
        _ => {
            println!("Invalid Opponent Choice. Setting to AI - EASY");
            oppo_choice = 1;
        }
    }

    let mut TOOT = TootAndOtto::new();

    let mut player = Player::TOOT;

    println!("{}", TOOT);

    loop 
    {
        println!("--> {} Player Turn <--", player);
        println!("Enter <T-O> <0-1-2-3-4-5>");

        let mut column: usize;
        let mut letter: char;
        let mut validation = false;

        if (oppo_choice == 0) || (player == Player::TOOT) 
        {
            while !validation 
            {
                let result = std_catch_OT();
                letter = result.0;
                column = result.1;
                validation = result.2;

                if !validation 
                {
                    continue;
                }

                let drop_piece = match letter 
                {
                    'T' => TOenum::T,
                    'O' => TOenum::O,
                    _ => 
                    {
                        println!("Invalid Input, Try Again");
                        continue;
                    }
                };

                validation = TOOT.drop(drop_piece, column);

                if !validation 
                {
                    println!("Invalid Input, Try Again");
                }

            }
        } 
        else 
        {
            let obs = toot_ai::AI_next_move(TOOT.clone(), oppo_choice);
            TOOT.drop(obs.1, obs.0);
        }

        println!("{}", TOOT);

        player = player.switch();

        if TOOT.termination 
        {
            break;
        }
    }

    match TOOT.winner 
    {
        Some(player) => 
        {
            println!("----------------------------------------------------");
            println!("{} Player Won!", player);
        }
        None => 
        {
            println!("----------------------------------------------------");
            println!("Drawn!");
        }
    }
}

fn main() 
{

    println!("Please choose following game to play: ");
    println!("1 -- Connect 4");
    println!("2 -- TOOT and OTTO");

    let mut input = String::new();

    if let Err(_) = io::stdin().read_line(&mut input) 
    {
        println!("Input failed, try again");
        return;
    };

    println!("--------------------------------------------------");
    println!("1. HUMAN");
    println!("2. AI - EASY");
    println!("3. AI - MEDIUM");
    println!("4. AI - EXPERT");

    let mut oppo = String::new();

    if let Err(_) = io::stdin().read_line(&mut oppo) 
    {
        println!("Invalid input, please try again!");    
        return;
    };

    let input = input.trim();

    if input == "1" 
    {
        game_connect4(oppo.trim());
    } 
    else if input == "2" 
    {
        game_TOTO(oppo.trim());
    }
    else
    {
        println!("Invalid input, please try again!");    
    }
}
