use deck::Card;
use player::{create_all_game_players, create_dealer, create_player, Player};
use rand::Rng;

pub mod deck;
pub mod game;
pub mod player;
extern crate rand;

fn main() {
    game::game_start();

    let players = create_all_game_players();

    let deck = deck::create_deck();
    // for i in deck.iter().enumerate() {
    //     println!("deck {:?}", i);
    // }

    let mut deck_len = deck.iter().count();
    deal_cards(deck, deck_len, players);
}

fn deal_cards(deck: Vec<Card>, max: usize, players: Vec<Player>) {
    // random card to player
    // random card to dealer
    // random card to player
    // random card to dealer
    let dealer = players.get(player);

    let card_one = generate_random_number(1, max);
    println!("card_one {}", card_one);
}

fn generate_random_number(min: usize, max: usize) -> usize {
    rand::thread_rng().gen_range(min..max)
}
