use std::collections::HashMap;

use crate::{
    deck::{create_deck, Card},
    player::{create_dealer, create_player, Player},
};

pub fn game_start() {
    // Do you want to play
    // Your name
    // Later…how many players
    println!("Do you want to play Blackjack?")
}

// pub fn game_setup() -> Vec<(HashMap<String, Player>, Vec<Card>)> {
//     // create the players
//     // create the deck
//     // maybe return a tuple with players and deck

//     let player = create_player();
//     let dealer = create_dealer();

//     let mut players = HashMap::new();
//     players.insert(String::from("player"), player);
//     players.insert(String::from("player"), dealer);

//     let mut deck: Vec<Card> = create_deck();

//     let response = Vec::new();
//     response.push(players);
//     response.push(deck);
//     response
// }

pub fn game_play() {
    // the act of playing the game, switching turns and such
}
