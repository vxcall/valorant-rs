// this is telling cargo what this directory contains
mod pre_game;
mod models;

/* 
    without this user have to do
    use valorant_rs::current_game::current_game::get_current_game_match;
    without
    use valorant_rs::current_game::get_current_game_match;
*/

pub use models::*;