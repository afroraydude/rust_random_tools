use rand::Rng;

struct Deck {
    cards: Vec<Card>,
}

struct Card {
    suit: String,
    pub value: String,
}

impl Deck {
    pub fn new() -> Deck {
        let mut cards = Vec::new();
        let suits = vec!["Hearts", "Diamonds", "Clubs", "Spades"];
        let values = vec![
            "Ace", "2", "3", "4", "5", "6", "7", "8", "9", "10", "Jack", "Queen", "King",
        ];

        for suit in suits {
            for value in &values {
                cards.push(Card {
                    suit: suit.to_string(),
                    value: value.to_string(),
                });
            }
        }

        Deck { cards }
    }

    pub fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        for i in 0..self.cards.len() {
            let j = rng.gen_range(0..self.cards.len());
            self.cards.swap(i, j);
        }
    }

    pub fn draw(&mut self) -> Card {
        self.cards.pop().unwrap()
    }

    pub fn print(&self) {
        for card in &self.cards {
            card.print();
        }
    }

    pub fn return_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn return_cards(&mut self, cards: Vec<Card>) {
        for card in cards {
            self.return_card(card);
        }
    }
}

impl Card {
    pub fn print(&self) {
        println!("{} of {}", self.value, self.suit);
    }

    pub fn get_value(&self) -> u8 {
        match self.value.as_str() {
            "2" => 2,
            "3" => 3,
            "4" => 4,
            "5" => 5,
            "6" => 6,
            "7" => 7,
            "8" => 8,
            "9" => 9,
            "10" => 10,
            "Jack" => 10,
            "Queen" => 10,
            "King" => 10,
            _ => 0,
        }
    }
}

impl Clone for Card {
    fn clone(&self) -> Self {
        Card {
            suit: self.suit.clone(),
            value: self.value.clone(),
        }
    }
}

fn print_scores(player_score: u8, dealer_exposed_card: Card) {
    println!("Player score: {}", player_score);
    print!("Dealer exposed card: ");
    dealer_exposed_card.print();
}

pub(crate) fn play() {
    println!("Welcome to blackjack!");
    println!("Starting game...");
    let mut deck = Deck::new();
    deck.shuffle();
    println!("Shuffled deck. Drawing cards...");

    let mut player_hand = Vec::new();
    let mut dealer_hand = Vec::new();

    player_hand.push(deck.draw());
    dealer_hand.push(deck.draw());
    player_hand.push(deck.draw());
    dealer_hand.push(deck.draw());

    println!("Player hand:");
    for card in &player_hand {
        card.print();
    }
    println!();

    println!(r#"Dealer hand:"#);
    &dealer_hand[0].print();
    println!("Hidden card");
    println!();

    let mut player_score = 0;
    let mut dealer_score = 0;

    for card in &player_hand {
        if (card.value == "Ace") {
            if (player_score + 11 > 21) {
                player_score += 1;
            } else {
                player_score += 11;
            }
        } else {
            player_score += card.get_value();
        }
    }

    for card in &dealer_hand {
        if card.value == "Ace" {
            if dealer_score + 11 > 21 {
                dealer_score += 1;
            } else {
                dealer_score += 11;
            }
        } else {
            dealer_score += card.get_value();
        }
    }

    // allow player to hit
    loop {
        print_scores(player_score, dealer_hand[0].clone());
        println!();
        println!("Hit or stand? (h/s)");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "h" {
            let new_card = deck.draw();
            new_card.print();
            if new_card.value == "Ace" {
                if player_score + 11 > 21 {
                    player_score += 1;
                } else {
                    player_score += 11;
                }
            } else {
                player_score += new_card.get_value();
            }

            player_hand.push(new_card);
        } else if input == "s" {
            break;
        } else {
            println!("Invalid input");
        }

        if player_score > 21 {
            println!("Player score: {}", player_score);
            println!("Player busts");

            println!("Play again? (y/n)");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();
            if input == "y" {
                play();
            } else {
                break;
            }
        }
    }

    if player_score == 21 {
        println!("Player score: {}", player_score);
        println!("Player wins");
        deck.return_cards(player_hand);
        deck.return_cards(dealer_hand);

        println!("Play again? (y/n)");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        if input == "y" {
            play();
        } else {
            return;
        }
    } else {
        println!("Player score: {}", player_score);
        println!("Dealer score: {}", dealer_score);
        println!("Dealer hand:");
        for card in &dealer_hand {
            card.print();
        }
        println!();
    }
}