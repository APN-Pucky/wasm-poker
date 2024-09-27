
#![allow(non_snake_case)]
use std::panic;

use wasm_bindgen::prelude::*;
use web_sys::console;

// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;

extern crate rs_poker;
use rs_poker::core::{Card, Hand, Suit, Value};
use rs_poker::holdem::MonteCarloGame;
use rand::Rng;


// A function to generate a random card
    fn random_hand() -> Hand{
        Hand::new_with_cards(vec![random_card(), random_card()])
    }

    // A function to generate a random card
    fn random_card() -> Card{
        Card {
            value: random_value(),
            suit: random_suit(),
        }
    }

    // A function to get a random value from the `Value` enum
    fn random_value() -> Value{
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..12) {
            0 => Value::Two,
            1 => Value::Three,
            2 => Value::Four,
            3 => Value::Five,
            4 => Value::Six,
            5 => Value::Seven,
            6 => Value::Eight,
            7 => Value::Nine,
            8 => Value::Ten,
            9 => Value::Jack,
            10 => Value::Queen,
            11 => Value::King,
            _ => Value::Ace,
        }
    }

    // A function to get a random suit from the `Suit` enum
    fn random_suit() -> Suit{
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..4) {
            0 => Suit::Heart,
            1 => Suit::Diamond,
            2 => Suit::Club,
            _ => Suit::Spade,
        }
    }

    fn overlap(h1: &Hand, h2: &Hand) -> bool {
        h1.iter().any(|c1| h2.iter().any(|c2| c1 == c2))
    }

    fn overlaps(h1: &Hand, h2: &Vec<Hand>) -> bool {
        h2.iter().any(|h| overlap(h1, h))
    }

    
    fn main() {
        panic::set_hook(Box::new(console_error_panic_hook::hook));
        // launch the web app
        launch(app);
    }

    fn vec_to_string<T: std::fmt::Debug>(vec: Vec<T>) -> String {
        // Convert each f64 to a String and join with ", "
        vec.iter()
           .map(|x| format!("{:?}", x)) // Format each float to 2 decimal places
           .collect::<Vec<String>>()      // Collect formatted strings into a Vec<String>
           .join(", ")                    // Join them with ", " as separator
    }

    fn hand_to_string(hand: Hand) -> String {
        // Convert each f64 to a String and join with ", "
        hand.iter()
           .map(|x| format!("{:?}", x)) // Format each float to 2 decimal places
           .collect::<Vec<String>>()      // Collect formatted strings into a Vec<String>
           .join(", ")                    // Join them with ", " as separator
    }
    
    // create a component that renders a div with the text "Hello, world!"
    fn app() -> Element {

        let mut selected_choice_suit1 = use_signal( || Suit::Heart); // defaults here are same as below ordering !!!
        let mut selected_choice_value1 = use_signal( || Value::Two); // defaults here are same as below ordering !!!

        let mut selected_choice_suit2 = use_signal(|| Suit::Heart); // defaults here are same as below ordering !!!
        let mut selected_choice_value2 = use_signal(|| Value::Two); // defaults here are same as below ordering !!!


        let mut public_choice_suit1  : Signal<Option<Suit>>= use_signal( || None);
        let mut public_choice_value1 : Signal<Option<Value>> = use_signal( || None);

        let mut public_choice_suit2  : Signal<Option<Suit>>= use_signal( || None);
        let mut public_choice_value2 : Signal<Option<Value>> = use_signal( || None);

        let mut public_choice_suit3  : Signal<Option<Suit>>= use_signal( || None);
        let mut public_choice_value3 : Signal<Option<Value>> = use_signal( || None);

        let mut public_choice_suit4  : Signal<Option<Suit>>= use_signal( || None);
        let mut public_choice_value4 : Signal<Option<Value>> = use_signal( || None);

        let mut public_choice_suit5  : Signal<Option<Suit>>= use_signal( || None);
        let mut public_choice_value5 : Signal<Option<Value>> = use_signal( || None);

        let mut count = use_signal(|| 4);
        let mut iters= use_signal(|| 100);

        let GAMES_COUNT: i32 = iters();
        let PLAYERS = count() as usize;
        //const PLAYERS: usize = count;
        let mut wins = vec![0; PLAYERS];
        let hand = Hand::new_with_cards(vec![Card {
                    value: selected_choice_value1(),
                    suit: selected_choice_suit1(),
                }, Card {
                    value: selected_choice_value2(),
                    suit: selected_choice_suit2(),
                }]);
        //let mut wins: [u64; count] = [0; count];
        for _ in 0..GAMES_COUNT {
            let mut hands = vec![hand.clone()];;
            // Add public cards
            if let Some(suit) = public_choice_suit1() {
                if let Some(value) = public_choice_value1() {
                    hands[0].push(Card {
                        value: value,
                        suit: suit,
                    });
                }
            }
            if let Some(suit) = public_choice_suit2() {
                if let Some(value) = public_choice_value2() {
                    hands[0].push(Card {
                        value: value,
                        suit: suit,
                    });
                }
            }
            if let Some(suit) = public_choice_suit3() {
                if let Some(value) = public_choice_value3() {
                    hands[0].push(Card {
                        value: value,
                        suit: suit,
                    });
                }
            }
            if let Some(suit) = public_choice_suit4() {
                if let Some(value) = public_choice_value4() {
                    hands[0].push(Card {
                        value: value,
                        suit: suit,
                    });
                }
            }
            if let Some(suit) = public_choice_suit5() {
                if let Some(value) = public_choice_value5() {
                    hands[0].push(Card {
                        value: value,
                        suit: suit,
                    });
                }
            }

            while hands.len() < PLAYERS {
                let h = random_hand();
                if !overlaps(&h, &hands) {
                    hands.push(h);
                }
            }

            // copy from hand[0] public cards to other hands
            let (first, rest) = hands.split_at_mut(1);
            for hand in rest.iter_mut() {
                for card in first[0].iter().skip(2) {
                    hand.push(*card);
                }
            };
            // panic!("Hands = {:?},{:?}", hands.iter().map(|a| a.len()).collect::<Vec<_>>(), hands[0].len());
            // assert length of hands same
            assert!( ! hands.iter().any(|h| h.len() != hands[0].len()));
            
            //console::log_1(&format!("Hands = {:?}", hands).into());

            //println!("Hands = {:?}", hands);
            let mut g = MonteCarloGame::new(hands).expect("Should be able to create a game.");
            let r = g.simulate();
            g.reset();
            wins[r.0.ones().next().unwrap()] += 1
        }

    let normalized: Vec<f64> = wins
        .iter()
        .map(|cnt| *cnt as f64 / GAMES_COUNT as f64 * 100.0)
        .collect();


        rsx! {
            h1 { "Player counter: {count}" }
            button { onclick: move |_| count += 1, "Up player!" }
            button { onclick: move |_| count -= 1, "Down player!" }
            p{}
            // label as iterations
            label { "Iterations: " }
            input {
                r#type: "number",
                value: "{iters}",
                oninput: move |e| {
                    let value = e.value();
                    iters.set(value.parse().unwrap_or(100));
                },
            }
            p{}
            label { "Your cards: " }
            p{}
            select {
                onchange: move |e| {
                    let value = e.value();
                    selected_choice_suit1.set(match value.as_str() {
                        "Heart" => Suit::Heart,
                        "Diamond" => Suit::Diamond,
                        "Club" => Suit::Club,
                        "Spade" => Suit::Spade,
                        _ => Suit::Heart,
                    });
                },
                //value: selected_choice_suit1,
                option { value: "Heart", "Heart" }
                option { value: "Diamond", "Diamond" }
                option { value: "Club", "Club" }
                option { value: "Spade", "Spade" }
            }
            select {
                onchange: move |e| {
                    let value = e.value();
                    selected_choice_value1.set(match value.as_str() {
                        "Two" => Value::Two,
                        "Three" => Value::Three,
                        "Four" => Value::Four,
                        "Five" => Value::Five,
                        "Six" => Value::Six,
                        "Seven" => Value::Seven,
                        "Eight" => Value::Eight,
                        "Nine" => Value::Nine,
                        "Ten" => Value::Ten,
                        "Jack" => Value::Jack,
                        "Queen" => Value::Queen,
                        "King" => Value::King,
                        "Ace" => Value::Ace,
                        _ => Value::Two,
                    });
                },
                //value: selected_choice_value1,
                option { value: "Two", "Two" }
                option { value: "Three", "Three" }
                option { value: "Four", "Four" }
                option { value: "Five", "Five" }
                option { value: "Six", "Six" }
                option { value: "Seven", "Seven" }
                option { value: "Eight", "Eight" }
                option { value: "Nine", "Nine" }
                option { value: "Ten", "Ten" }
                option { value: "Jack", "Jack" }
                option { value: "Queen", "Queen" }
                option { value: "King", "King" }
                option { value: "Ace", "Ace" }
            }

           p{}
            select {
                onchange: move |e| {
                    let value = e.value();
                    selected_choice_suit2.set(match value.as_str() {
                        "Heart" => Suit::Heart,
                        "Diamond" => Suit::Diamond,
                        "Club" => Suit::Club,
                        "Spade" => Suit::Spade,
                        _ => Suit::Heart,
                    });
                },
                //value: selected_choice_suit1,
                option { value: "Heart", "Heart" }
                option { value: "Diamond", "Diamond" }
                option { value: "Club", "Club" }
                option { value: "Spade", "Spade" }
            }
            select {
                onchange: move |e| {
                    let value = e.value();
                    selected_choice_value2.set(match value.as_str() {
                        "Two" => Value::Two,
                        "Three" => Value::Three,
                        "Four" => Value::Four,
                        "Five" => Value::Five,
                        "Six" => Value::Six,
                        "Seven" => Value::Seven,
                        "Eight" => Value::Eight,
                        "Nine" => Value::Nine,
                        "Ten" => Value::Ten,
                        "Jack" => Value::Jack,
                        "Queen" => Value::Queen,
                        "King" => Value::King,
                        "Ace" => Value::Ace,
                        _ => Value::Two,
                    });
                },
                //value: selected_choice_value1,
                option { value: "Two", "Two" }
                option { value: "Three", "Three" }
                option { value: "Four", "Four" }
                option { value: "Five", "Five" }
                option { value: "Six", "Six" }
                option { value: "Seven", "Seven" }
                option { value: "Eight", "Eight" }
                option { value: "Nine", "Nine" }
                option { value: "Ten", "Ten" }
                option { value: "Jack", "Jack" }
                option { value: "Queen", "Queen" }
                option { value: "King", "King" }
                option { value: "Ace", "Ace" }
            }
            p{}
            label { "Public cards: " }
            p{}
            select {
                onchange: move |e| {
                    let value = e.value();
                    public_choice_suit1.set(match value.as_str() {
                        "Heart" => Some(Suit::Heart),
                        "Diamond" => Some(Suit::Diamond),
                        "Club" => Some(Suit::Club),
                        "Spade" => Some(Suit::Spade),
                        _ => None,
                    });
                },
                //value: selected_choice_suit1,
                option { value: "None", "None" }
                option { value: "Heart", "Heart" }
                option { value: "Diamond", "Diamond" }
                option { value: "Club", "Club" }
                option { value: "Spade", "Spade" }
            }
            select {
                onchange: move |e| {
                    let value = e.value();
                    public_choice_value1.set(match value.as_str() {
                        "Two" => Some(Value::Two),
                        "Three" => Some(Value::Three),
                        "Four" => Some(Value::Four),
                        "Five" => Some(Value::Five),
                        "Six" => Some(Value::Six),
                        "Seven" => Some(Value::Seven),
                        "Eight" => Some(Value::Eight),
                        "Nine" => Some(Value::Nine),
                        "Ten" => Some(Value::Ten),
                        "Jack" => Some(Value::Jack),
                        "Queen" => Some(Value::Queen),
                        "King" => Some(Value::King),
                        "Ace" => Some(Value::Ace),
                        _ => None,
                    });
                },
                //value: selected_choice_value1,
                option { value: "None", "None" }
                option { value: "Two", "Two" }
                option { value: "Three", "Three" }
                option { value: "Four", "Four" }
                option { value: "Five", "Five" }
                option { value: "Six", "Six" }
                option { value: "Seven", "Seven" }
                option { value: "Eight", "Eight" }
                option { value: "Nine", "Nine" }
                option { value: "Ten", "Ten" }
                option { value: "Jack", "Jack" }
                option { value: "Queen", "Queen" }
                option { value: "King", "King" }
                option { value: "Ace", "Ace" }
            }
            p {}
            select {
                onchange: move |e| {
                    let value = e.value();
                    public_choice_suit2.set(match value.as_str() {
                        "Heart" => Some(Suit::Heart),
                        "Diamond" => Some(Suit::Diamond),
                        "Club" => Some(Suit::Club),
                        "Spade" => Some(Suit::Spade),
                        _ => None,
                    });
                },
                //value: selected_choice_suit1,
                option { value: "None", "None" }
                option { value: "Heart", "Heart" }
                option { value: "Diamond", "Diamond" }
                option { value: "Club", "Club" }
                option { value: "Spade", "Spade" }
            }
            select {
                onchange: move |e| {
                    let value = e.value();
                    public_choice_value2.set(match value.as_str() {
                        "Two" => Some(Value::Two),
                        "Three" => Some(Value::Three),
                        "Four" => Some(Value::Four),
                        "Five" => Some(Value::Five),
                        "Six" => Some(Value::Six),
                        "Seven" => Some(Value::Seven),
                        "Eight" => Some(Value::Eight),
                        "Nine" => Some(Value::Nine),
                        "Ten" => Some(Value::Ten),
                        "Jack" => Some(Value::Jack),
                        "Queen" => Some(Value::Queen),
                        "King" => Some(Value::King),
                        "Ace" => Some(Value::Ace),
                        _ => None,
                    });
                },
                //value: selected_choice_value1,
                option { value: "None", "None" }
                option { value: "Two", "Two" }
                option { value: "Three", "Three" }
                option { value: "Four", "Four" }
                option { value: "Five", "Five" }
                option { value: "Six", "Six" }
                option { value: "Seven", "Seven" }
                option { value: "Eight", "Eight" }
                option { value: "Nine", "Nine" }
                option { value: "Ten", "Ten" }
                option { value: "Jack", "Jack" }
                option { value: "Queen", "Queen" }
                option { value: "King", "King" }
                option { value: "Ace", "Ace" }
            }
            p{}
            select {
                onchange: move |e| {
                    let value = e.value();
                    public_choice_suit3.set(match value.as_str() {
                        "Heart" => Some(Suit::Heart),
                        "Diamond" => Some(Suit::Diamond),
                        "Club" => Some(Suit::Club),
                        "Spade" => Some(Suit::Spade),
                        _ => None,
                    });
                },
                //value: selected_choice_suit1,
                option { value: "None", "None" }
                option { value: "Heart", "Heart" }
                option { value: "Diamond", "Diamond" }
                option { value: "Club", "Club" }
                option { value: "Spade", "Spade" }
            }
            select {
                onchange: move |e| {
                    let value = e.value();
                    public_choice_value3.set(match value.as_str() {
                        "Two" => Some(Value::Two),
                        "Three" => Some(Value::Three),
                        "Four" => Some(Value::Four),
                        "Five" => Some(Value::Five),
                        "Six" => Some(Value::Six),
                        "Seven" => Some(Value::Seven),
                        "Eight" => Some(Value::Eight),
                        "Nine" => Some(Value::Nine),
                        "Ten" => Some(Value::Ten),
                        "Jack" => Some(Value::Jack),
                        "Queen" => Some(Value::Queen),
                        "King" => Some(Value::King),
                        "Ace" => Some(Value::Ace),
                        _ => None,
                    });
                },
                //value: selected_choice_value1,
                option { value: "None", "None" }
                option { value: "Two", "Two" }
                option { value: "Three", "Three" }
                option { value: "Four", "Four" }
                option { value: "Five", "Five" }
                option { value: "Six", "Six" }
                option { value: "Seven", "Seven" }
                option { value: "Eight", "Eight" }
                option { value: "Nine", "Nine" }
                option { value: "Ten", "Ten" }
                option { value: "Jack", "Jack" }
                option { value: "Queen", "Queen" }
                option { value: "King", "King" }
                option { value: "Ace", "Ace" }
            }
            p{}
            select {
                onchange: move |e| {
                    let value = e.value();
                    public_choice_suit4.set(match value.as_str() {
                        "Heart" => Some(Suit::Heart),
                        "Diamond" => Some(Suit::Diamond),
                        "Club" => Some(Suit::Club),
                        "Spade" => Some(Suit::Spade),
                        _ => None,
                    });
                },
                //value: selected_choice_suit1,
                option { value: "None", "None" }
                option { value: "Heart", "Heart" }
                option { value: "Diamond", "Diamond" }
                option { value: "Club", "Club" }
                option { value: "Spade", "Spade" }
            }
            select {
                onchange: move |e| {
                    let value = e.value();
                    public_choice_value4.set(match value.as_str() {
                        "Two" => Some(Value::Two),
                        "Three" => Some(Value::Three),
                        "Four" => Some(Value::Four),
                        "Five" => Some(Value::Five),
                        "Six" => Some(Value::Six),
                        "Seven" => Some(Value::Seven),
                        "Eight" => Some(Value::Eight),
                        "Nine" => Some(Value::Nine),
                        "Ten" => Some(Value::Ten),
                        "Jack" => Some(Value::Jack),
                        "Queen" => Some(Value::Queen),
                        "King" => Some(Value::King),
                        "Ace" => Some(Value::Ace),
                        _ => None,
                    });
                },
                //value: selected_choice_value1,
                option { value: "None", "None" }
                option { value: "Two", "Two" }
                option { value: "Three", "Three" }
                option { value: "Four", "Four" }
                option { value: "Five", "Five" }
                option { value: "Six", "Six" }
                option { value: "Seven", "Seven" }
                option { value: "Eight", "Eight" }
                option { value: "Nine", "Nine" }
                option { value: "Ten", "Ten" }
                option { value: "Jack", "Jack" }
                option { value: "Queen", "Queen" }
                option { value: "King", "King" }
                option { value: "Ace", "Ace" }
            }
            p{}
            select {
                onchange: move |e| {
                    let value = e.value();
                    public_choice_suit5.set(match value.as_str() {
                        "Heart" => Some(Suit::Heart),
                        "Diamond" => Some(Suit::Diamond),
                        "Club" => Some(Suit::Club),
                        "Spade" => Some(Suit::Spade),
                        _ => None,
                    });
                },
                //value: selected_choice_suit1,
                option { value: "None", "None" }
                option { value: "Heart", "Heart" }
                option { value: "Diamond", "Diamond" }
                option { value: "Club", "Club" }
                option { value: "Spade", "Spade" }
            }
            select {
                onchange: move |e| {
                    let value = e.value();
                    public_choice_value5.set(match value.as_str() {
                        "Two" => Some(Value::Two),
                        "Three" => Some(Value::Three),
                        "Four" => Some(Value::Four),
                        "Five" => Some(Value::Five),
                        "Six" => Some(Value::Six),
                        "Seven" => Some(Value::Seven),
                        "Eight" => Some(Value::Eight),
                        "Nine" => Some(Value::Nine),
                        "Ten" => Some(Value::Ten),
                        "Jack" => Some(Value::Jack),
                        "Queen" => Some(Value::Queen),
                        "King" => Some(Value::King),
                        "Ace" => Some(Value::Ace),
                        _ => None,
                    });
                },
                //value: selected_choice_value1,
                option { value: "None", "None" }
                option { value: "Two", "Two" }
                option { value: "Three", "Three" }
                option { value: "Four", "Four" }
                option { value: "Five", "Five" }
                option { value: "Six", "Six" }
                option { value: "Seven", "Seven" }
                option { value: "Eight", "Eight" }
                option { value: "Nine", "Nine" }
                option { value: "Ten", "Ten" }
                option { value: "Jack", "Jack" }
                option { value: "Queen", "Queen" }
                option { value: "King", "King" }
                option { value: "Ace", "Ace" }
            }
            div {
                p {"Starting Hand = {hand_to_string(hand)}"}
                p {"Wins [#] = {vec_to_string(wins)}"}
                p {"Normalized Wins [%] = {vec_to_string(normalized)}"}
            }
        }
    }


/// Tests module
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_overlap() {
        let h1 = Hand::new_from_str("TdTh").unwrap();
        let h2 = Hand::new_from_str("9d9h").unwrap();
        let h3 = Hand::new_from_str("9d9h").unwrap();
        let h4 = Hand::new_from_str("9h9d").unwrap();
        let h5 = Hand::new_from_str("ThTd").unwrap();

        assert_eq!(overlap(&h1, &h2), false);
        assert_eq!(overlap(&h2, &h3), true);
        assert_eq!(overlap(&h3, &h4), true);
        assert_eq!(overlap(&h3, &h5), false);
        assert_eq!(overlap(&h1, &h5), true);


        assert_eq!(overlap(&h1, &h1), true);
        assert_eq!(overlap(&h2, &h2), true);
        assert_eq!(overlap(&h3, &h3), true);
        assert_eq!(overlap(&h4, &h4), true);
    }
}