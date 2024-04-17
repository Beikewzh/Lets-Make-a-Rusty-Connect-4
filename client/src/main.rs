#![recursion_limit = "1024"]

mod app;
mod switch;

mod components {
    pub mod connect4_page;
    pub mod auth;
    pub mod sidebar;
    pub mod stats;
    pub mod toot_and_otto_page;
}

mod connect4 {
    pub mod connect4;
    pub mod con4_ai;
}

mod toot_and_otto {
    pub mod toot_ai;
    pub mod toot_and_otto;
}

mod types {
    pub mod opponent;
}

use wasm_logger;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<app::App>();
}
