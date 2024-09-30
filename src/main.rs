pub mod deck;
pub mod game;

fn main() {
    game::game_start();
    // let first_card = deck::FIVE;
    // println!("first_card: {:?}", first_card);

    // let second_card = deck::SIX;
    // println!("second_card: {:?}", second_card);

    // let total = first_card + second_card;
    // // let total = 5 + 5;
    // println!("total: {}", total);
    let deck = deck::create_deck();
    for i in deck.iter().enumerate() {
        println!("deck {:?}", i);
    }
}
