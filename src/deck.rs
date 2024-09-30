#[derive(Debug)]
pub struct Card {
    name: String,
    value: u8,
    suit: String,
}

pub fn create_deck() -> Vec<Card> {
    let suits = ["spade", "club", "heart", "diamond"];

    let values: [u8; 13] = [2, 3, 4, 5, 6, 7, 8, 9, 10, 10, 10, 10, 11];

    let names = [
        "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten", "jack", "queen",
        "king", "ace",
    ];

    let mut deck: Vec<Card> = Vec::new();

    for suit in suits {
        for index in 0..values.len() {
            deck.push(Card {
                name: String::from(names[index]),
                value: values[index],
                suit: String::from(suit),
            });
        }
    }

    deck
}
