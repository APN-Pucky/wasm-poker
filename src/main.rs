
#![allow(non_snake_case)]
use std::panic;

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

    fn array_to_string<T: std::fmt::Debug, const N: usize>(vec: [T; N]) -> String {
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

        let mut selected_choice_suit1 = use_signal( || Suit::Heart);
        let mut selected_choice_value1 = use_signal( || Value::King);
        let mut selected_choice_suit2 = use_signal(|| Suit::Spade);
        let mut selected_choice_value2 = use_signal(|| Value::King);

        let mut count = use_signal(|| 4);

        const GAMES_COUNT: i32 = 100;
        let PLAYERS = count() as usize;
        //const PLAYERS: usize = count;
        let shand = "TdTh";
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
            let mut hands = vec![
                hand.clone()
                ];
            while hands.len() < PLAYERS {
                let h = random_hand();
                if !overlaps(&h, &hands) {
                    hands.push(h);
                }
            }
            //println!("Hands = {:?}", hands);
            let mut g = MonteCarloGame::new(hands).expect("Should be able to create a game.");
            let r = g.simulate();
            g.reset();
            wins[r.0.ones().next().unwrap()] += 1
        }

    let normalized: Vec<f64> = wins
        .iter()
        .map(|cnt| *cnt as f64 / GAMES_COUNT as f64)
        .collect();


        rsx! {
            h1 { "Player counter: {count}" }
            button { onclick: move |_| count += 1, "Up player!" }
            button { onclick: move |_| count -= 1, "Down player!" }
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
                option { value: "Hear", "Heart" }
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
                option { value: "Hear", "Heart" }
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
            div {
                "Hello, world!"
                p {"Starting Hand =\t{hand_to_string(hand)}"}
                p {"Wins ={vec_to_string(wins)}"}
                p {"Normalized Wins ={vec_to_string(normalized)}"}
            }
        }
    }

fn main2() {

    const GAMES_COUNT: i32 = 3000;
    const PLAYERS: usize = 4;
    let shand = "TdTh";
    let mut wins: [u64; PLAYERS] = [0; PLAYERS];
    for _ in 0..GAMES_COUNT {
        let mut hands = vec![Hand::new_from_str(shand).expect("Hand should be valid")];
        while hands.len() < PLAYERS {
            let h = random_hand();
            if !overlaps(&h, &hands) {
                hands.push(h);
            }
        }
        //println!("Hands = {:?}", hands);
        let mut g = MonteCarloGame::new(hands).expect("Should be able to create a game.");
        let r = g.simulate();
        g.reset();
        wins[r.0.ones().next().unwrap()] += 1
    }

    let normalized: Vec<f64> = wins
        .iter()
        .map(|cnt| *cnt as f64 / GAMES_COUNT as f64)
        .collect();

    println!("Starting Hand =\t{:?}", shand);
    println!("Wins =\t\t\t{:?}", wins);
    println!("Normalized Wins =\t{:?}", normalized);
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