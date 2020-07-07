    use std::io::{self, Write};
    use colored::Colorize;
    use crate::decking;
    use std::collections::HashMap;
    use crate::splash_screen;

pub fn game(){
    let mut total_chips = 100_i32;
    splash_screen::print_front();
    loop{
        let mut d = decking::Deck::new();
        let mut hand: Vec<decking::Card> = Vec::new();
        let mut dealer: Vec<decking::Card> = Vec::new();
        let mut input_txt = String::new();
        let mut total_bet = 0_i32;
        #[allow(unused_assignments)]
        let mut current_bet = 0_i32;
        let mut bet_limit = total_chips;
        
        for _i in 0..2{
            hand.push(d.get_card());
        }

        for _d in 0..5{
            dealer.push(d.get_card());
        }

        let show_none = || {
            let nn = format!("{}", "   ".green().on_green());
            println!("\t{0} {0} {0} {0} {0}\n\t{0} {0} {0} {0} {0}",nn);
        };

        let hide_four = || {
            let nn = format!("{}", "   ".green().on_green());
            println!("\t{0} {0} {0} {0}\n\t{0} {0} {0} {0}",nn);
        };

        let hide_three = || {
            let nn = format!("{}", "   ".green().on_green());
            println!("\t{0} {0} {0} \n\t{0} {0} {0}",nn);
        };



        let show_dealer = |n:i32| {
            if n == 1 {
                hide_four();
                println!("{}", dealer[0]);
            } else if n == 2 {
                hide_three();
                println!("{} {}", dealer[0], dealer[1]);
            } else if n == 5 {
                println!("{} {} {} {} {}", dealer[0], dealer[1], dealer[2], dealer[3], dealer[4]);
            } else {
                show_none();
            }
        };




        let show_hand = || {
            for cd in hand.iter(){
                println!("{}", cd);
            }
        };

        //input_txt = String::from("");
        let dirns = format!("{}","\n--- Dealer's cards ▲▲▲---\n--- Your cards ▼▼▼---\n".yellow());
        for round in 0..3{
            std::process::Command::new("clear").status().expect("Could not clear");

            let top_display = format!("\n Total chips: {}   Bet limit: {}    Total bet: {} \n\n", total_chips, bet_limit, total_bet);
            println!("   {}", top_display.yellow().bold().on_black());

            show_dealer(round);
            println!("{}",dirns);
            show_hand();

            loop{
            input_txt = String::from("");
            
            print!("Enter bet (limit {}): ", bet_limit);
            io::stdout().flush().unwrap();
            io::stdin()
                .read_line(&mut input_txt)
                .unwrap();


            let temp = &(input_txt.trim()).clone();
            
            if temp == &"quit" {
                std::process::exit(0);
            }

            current_bet = temp.parse::<i32>().unwrap_or(400_i32);

            if current_bet > bet_limit{
                println!("INVALID BET --try again");
                continue;
            }
            total_chips -= current_bet;
            total_bet += current_bet;
            if current_bet > total_chips{
                bet_limit = total_chips;
            } else {
                bet_limit = current_bet;
            }
            println!("\nTotal chips {}  Total bet {}  Current bet {}  Bet limit {}", total_chips, total_bet, current_bet, bet_limit);

            println!("Press Enter");
            io::stdin()
                .read_line(&mut input_txt)
                .unwrap();

            break;
            }

        }
        std::process::Command::new("clear").status().expect("Could not clear");

        show_dealer(5);
        println!("{}",dirns);
        show_hand();
        let mut v_hash:HashMap<u32,u32> = HashMap::new();
        let mut h_hash:HashMap<String, u32> = HashMap::new();
        for dc in dealer.iter() {
            match dc.house{
                decking::House::Hearts => *h_hash.entry(String::from("hearts")).or_insert(0) += 1,
                decking::House::Clubs => *h_hash.entry(String::from("clubs")).or_insert(0) += 1,
                decking::House::Diamonds => *h_hash.entry(String::from("diamonds")).or_insert(0) += 1,
                decking::House::Spades => *h_hash.entry(String::from("spades")).or_insert(0) += 1,
            }
            match dc.val{
                decking::Value::Num(x) => *v_hash.entry(x as u32).or_insert(0) += 1,
                decking::Value::Ace => *v_hash.entry(1_u32).or_insert(0) += 1,
                decking::Value::Jack => *v_hash.entry(11_u32).or_insert(0) += 1,
                decking::Value::Queen => *v_hash.entry(12_u32).or_insert(0) += 1,
                decking::Value::King => *v_hash.entry(13_u32).or_insert(0) += 1,
            }
        }

        for hc in hand.iter() {
            match hc.house{
                decking::House::Hearts => *h_hash.entry(String::from("hearts")).or_insert(0) += 1,
                decking::House::Clubs => *h_hash.entry(String::from("clubs")).or_insert(0) += 1,
                decking::House::Diamonds => *h_hash.entry(String::from("diamonds")).or_insert(0) += 1,
                decking::House::Spades => *h_hash.entry(String::from("spades")).or_insert(0) += 1,
            }
            match hc.val{
                decking::Value::Num(x) => *v_hash.entry(x as u32).or_insert(0) += 1,
                decking::Value::Ace => *v_hash.entry(1_u32).or_insert(0) += 1,
                decking::Value::Jack => *v_hash.entry(11_u32).or_insert(0) += 1,
                decking::Value::Queen => *v_hash.entry(12_u32).or_insert(0) += 1,
                decking::Value::King => *v_hash.entry(13_u32).or_insert(0) += 1,
            }
        }


        //println!("{:?}", h_hash);
        //println!("{:?}", v_hash);

        let mut is_flush = false;
        let mut is_fullhouse = false;
        let mut is_quad = false;
        let mut is_tri = false;
        let mut is_double = false;
        let mut is_two_double = false;

        for x in &h_hash{
            if *x.1 == 5_u32 {
                is_flush = true;
            }
        }

        for y in &v_hash{
            if *y.1 == 4_u32 {
                is_quad = true;
                break;
            }
            if *y.1 == 3_u32 {
                if is_tri || is_double {
                    is_fullhouse = true;
                    break;
                }
                is_tri = true;
            }
            if *y.1 == 2_u32 {
                if is_tri {
                    is_fullhouse = true;
                    break;
                }
                if is_double {
                    is_two_double = true;
                    break;
                }
                is_double = true;
            }
        }
        let final_chips:f32;
        if is_flush {
            println!("FLUSH (180% bet amount)");
            final_chips = 1.8_f32 * (total_bet as f32);
        } else if is_quad {
            println!("FOUR OF A KIND (200% bet amount)");
            final_chips = 2.0_f32 * (total_bet as f32);
        } else if is_straight(v_hash){
            println!("STRAIGHT (160% bet amount)");
            final_chips = 1.6_f32 * (total_bet as f32);
        } else if is_fullhouse {
            println!("FULLHOUSE (150% bet amount)");
            final_chips = 1.5_f32 * (total_bet as f32);
        } else if is_tri {
            println!("THREE OF A KIND (120% bet amount)");
            final_chips = 1.2_f32 * (total_bet as f32);
        } else if is_two_double {
            println!("DOUBLE DOUBLES (100% bet amount- no gain or loss)");
            final_chips = 1.0_f32 * (total_bet as f32);
        } else if is_double {
            println!("DOUBLE (80% bet amount)");
            final_chips = 0.8_f32 * (total_bet as f32);
        } else {
            println!("you got nothing (lose all the bet amount)");
            final_chips = 0.0_f32;
        }

        println!("This round:");
        println!("started with {}; bet {}; remaining {}; won {} from bet amount", total_chips + total_bet, total_bet, total_chips, final_chips as i32);
        total_chips += final_chips as i32;
        println!("Total Chips Overall (won amount + remaining from start): {}", total_chips);

        if total_chips > 199 {
            splash_screen::you_win();
            break;
        }
        if total_chips < 10 {
            splash_screen::you_lose();
            break;
        }

        println!("Press Enter");
        io::stdin()
            .read_line(&mut input_txt)
            .unwrap();


    }
}

fn is_straight(val_hash:HashMap<u32, u32>) -> bool{
    let mut val:Vec<u32> = Vec::new();
    for (x,_) in val_hash{
        val.push(x);
    }
    val.sort();

    if val.len() < 5{
        return false;
    } else {
        for j in 0..3{
            if j == 1 && val.len() == 5 {
                return false;
            }
            if j == 2 && val.len() == 6 {
                return false;
            }
            if val[j+1] == (val[j] + 1) &&
               val[j+2] == (val[j+1] +1) &&
               val[j+3] == (val[j+2] +1) &&
               val[j+4] == (val[j+3] +1) {
               return true;
            }
            if val[j] == 10 &&
                val[j+1] == 11 &&
                val[j+2] == 12 &&
                val[j+3] == 13 &&
                val[0] == 1 {
                    return true;
            }
        }
    }
    return false;    
}
