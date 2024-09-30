use crate::deck::Card;
#[derive(Debug)]
enum PlayerIdentity {
    DEALER,
    PLAYER,
    OTHER_PLAYER,
}
#[derive(Debug)]
pub struct Player {
    name: String,
    hand: Option<Vec<Card>>,
    isActive: bool,
    identity: PlayerIdentity,
}

pub fn create_player() -> Player {
    Player {
        name: String::from("Dan"),
        hand: None,
        isActive: false,
        identity: PlayerIdentity::PLAYER,
    }
}

pub fn create_dealer() -> Player {
    Player {
        name: String::from("Bob"),
        hand: None,
        isActive: false,
        identity: PlayerIdentity::DEALER,
    }
}

pub fn create_all_game_players() -> Vec<Player> {
    let player = create_player();
    let dealer = create_dealer();
    vec![player, dealer]
}
