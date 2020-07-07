    use colored::Colorize;
    use std::fmt;
    use rand::{thread_rng, seq::SliceRandom};

    pub struct Deck{
        d: Vec<Card>,
    }

    impl Deck{

        pub fn new() -> Deck{

            let mut cardset:Vec<Card> = Vec::new();

            for v in 2..11{
                for h in [House::Hearts, House::Clubs, House::Diamonds, House::Spades].iter(){
                    cardset.push(Card{
                        house: *h,
                        val: Value::Num(v),
                    });
                }
            }

            for v in [Value::Ace, Value::King, Value::Queen, Value::Jack].iter(){
                for h in [House::Hearts, House::Clubs, House::Diamonds, House::Spades].iter(){
                    cardset.push(Card{
                        house: *h,
                        val: *v,
                    });
                }
            }

            cardset.shuffle(&mut thread_rng());


            Deck{
                d: cardset,
            }
        }

        pub fn get_card(&mut self) -> Card{
            self.d.pop().unwrap()
        }

    }

    pub struct Card{
       pub house: House,
       pub val: Value,
    }

    #[derive(Copy, Clone)]
    pub enum House{
        Hearts,
        Clubs,
        Diamonds,
        Spades,
    }

    #[derive(Copy, Clone)]
    pub enum Value{
        Num(i32),
        Ace,
        King,
        Queen,
        Jack,
    }

    impl fmt::Display for Card{
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
            write!(f, "\n{}", self.format_card())
        }
    }

    impl Card{

        fn format_value(&self) -> String{
            match self.val{
                Value::Num(x) => {
                    if x == 10 {
                        format!("{} ", x)
                    }
                    else {
                        format!("{}  ", x)
                    }
                },
                Value::Ace => format!("{}  ", "A"),
                Value::King => format!("{}  ", "K"),
                Value::Queen => format!("{}  ", "Q"),
                Value::Jack => format!("{}  ", "J"),
            }
        }

        fn format_card(&self) -> String{
            match self.house{
                House::Hearts => format!(" \t{}\n \t{}\n", self.format_value().white().on_red(), "  ♡".white().on_red()),
                House::Diamonds => format!(" \t{}\n \t{}\n", self.format_value().white().on_red(), "  ♢".white().on_red()),
                House::Clubs => format!(" \t{}\n \t{}\n", self.format_value().white().on_black(), "  ♣".white().on_black()),
                House::Spades => format!(" \t{}\n \t{}\n", self.format_value().white().on_black(), "  ♠".white().on_black()),
            }
        }

    }

