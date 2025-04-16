use rand::seq::SliceRandom;
use std::io;
use std::collections::HashMap;
use std::fmt;
use rand::Rng;
#[derive(Clone)]
#[derive(Debug)]
struct Card{
    value: u8,
    suit: String,
    color: u8
}
impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} of {}", self.value, self.suit)
    }

}
fn shuffle(deck:&mut Vec<Card>){
    let mut rng = rand::thread_rng();
    deck.shuffle(&mut rng);
}


fn wichsuit(a:i8) -> String{
    let suit;
    match a {
        1=>suit="clubs",
        2=>suit="diamonds",
        3=>suit="hearts",
        4=>suit="spades",
        _=>suit="joker"
    }
    return suit.to_string()
}
fn wichcolor(a:i8)->u8{
    let color;
    match a {
        1=>color=1,
        2=>color=2,
        3=>color=4,
        4=>color=3,
        _=>color=5,

    }
    return color
}

fn dealing (deck: Vec<Card>,i: usize,d: usize ) ->Vec<Card>{
    let mut card: Vec<Card>=Vec::new();
    card.push(deck[i+d].clone());
    return card;
}

fn combo(table: Vec<Card>,players:HashMap<String,Vec<Card>>,scores:&mut HashMap<String,u8>,playing:usize){
    let mut straight=0;
    let mut flush=0;
    for (key,value) in players {
        let mut on_hand :Vec<Card>=table.clone();
            on_hand.extend(value.clone());

    on_hand.sort_by(|a,b| b.value.cmp(&a.value));
    let mut combos : u8=0;
        for i in &on_hand{
        println!("{}: {} {}",key,i.value,i.suit )
        }


    for d in 0..on_hand.len() - 1 {
        if on_hand[d].value == on_hand[d + 1].value {
            combos=1;
            for _i in 0..on_hand.len()-1{
                if on_hand[_i].value!=on_hand[d].value&& on_hand[_i].value==on_hand[_i+1].value{
                    combos=combos+1;
                }
            }
        }
    }
    for d in 0..on_hand.len()-2{
        if on_hand[d].value == on_hand[d+1].value && on_hand[d+1].value == on_hand[d+2].value{
            combos=3;
        }
    }
    for d in 0..on_hand.len()-4{
        if on_hand[d+1].value-1==on_hand[d].value && on_hand[d+2].value-1==on_hand[d+1].value && on_hand[d+3].value-1==on_hand[d+2].value && on_hand[d+4].value-1==on_hand[d+3].value{
            combos=combos+4;
            straight=1;
        }
    }
    for d in 0..on_hand.len()-2{
        if on_hand[d].value==on_hand[d+1].value && on_hand[d+1].value==on_hand[d+2].value{
            for i in 0..on_hand.len()-1{
                if on_hand[i].value!=on_hand[d].value{
                    if on_hand[i].value==on_hand[i+1].value{
                        combos=6;
                    }
                }
            }
        }
    }
    for d in 0..on_hand.len()-3{
        if on_hand[d].value==on_hand[d+1].value && on_hand[d+1].value==on_hand[d+2].value && on_hand[d+2].value==on_hand[d+3].value{
            combos=combos+7;
        }
    }
    on_hand.sort_by(|a,b|a.color.cmp(&b.color));
    for d in 0..on_hand.len()-4{
        if on_hand[d].color==on_hand[d+1].color && on_hand[d+1].color==on_hand[d+2].color && on_hand[d+2].color==on_hand[d+3].color && on_hand[d+3].color==on_hand[d+4].color{
            combos=combos+5;
            flush=1;
        }
    }
    if flush==1 && straight==1{
        combos=8;
    }
    for d in 0..on_hand.len(){
        if on_hand[d].value==12 && straight==1 && flush==1{
            combos=9
        }
    }
    println!("{}",combos);
    scores.entry(key).and_modify(|v|*v=*v+combos);
    }
}


fn main(){
    let mut  its_raise_time :String="bet".to_string();
    let mut rng=rand::thread_rng();
    println!("Each player has 100 coins!");
    println!("Number of players (max 4):");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Couldn't read the data!");

    let number_of_players: usize = input
        .trim()
        .parse()
        .expect("It's not a right number!");

    if number_of_players > 4 {
        println!("Maximum number of players is 4.");
        return;
    }
    let mut betted : u8=0;
    let mut coins : HashMap<String,u8>=HashMap::new();
    for i in 0..number_of_players-1{
        let player_number=format!("player{}",i);
        coins.entry(player_number.to_string()).or_insert(100);
    }


    let mut s = 1;
    let mut d = 0;
    let mut deck: Vec<Card>=Vec::new();

    for _i in 0..52{
        d = d+1;
        if d == 14{
            d = 1;
            s = s+1;
        };
        let card= Card {
            value: d,
            suit: String::from(wichsuit(s)),
            color: wichcolor(s)
        };
        deck.push(card);
    };
    let mut betting = 0;

    shuffle(&mut deck);
    let mut table : Vec<Card>= Vec::new();
    let mut players : HashMap<String,Vec<Card>>= HashMap::new();
    let mut scores : HashMap<String,u8> = HashMap::new();
    let mut wich_deal: usize = 0;
    for i in 0..number_of_players-1{
        let player_number=format!("player{}",i);
        let card=dealing(deck.clone(),i,wich_deal);
        players.entry(player_number.to_string()).or_insert_with(Vec::new).extend(card);
        scores.entry(player_number.to_string()).or_insert(0);
    }
    wich_deal=wich_deal+5;
    for i in 0..number_of_players-1{
        let player_number=format!("player{}",i);
        let card=dealing(deck.clone(),i,wich_deal);
        players.entry(player_number.to_string()).or_insert_with(Vec::new).extend(card);
    }
    wich_deal=wich_deal+5;
    for _i in 0..2{
        table.extend(dealing(deck.clone(),0,wich_deal));
        wich_deal+=1
    }
    println!{"On the table:"}
    for i in 0..2{
        println!(" value: {} suit: {}, ",table[i].value,table[i].suit);
    }

    println!("Your Cards:");
    match players.get("player1") {
        Some(cards) => {
            println!("Your cards:");
            for (i, card) in cards.iter().enumerate() {
                println!("  {}. {}", i + 1, card);  // Uses Card's Display impl
            }
        },
        None => println!("Player not found!"),
    }
    let mut passed:usize=1;
    println!("what do you want to do?:");
    println!("1. Pas");
    println!("2. Check");
    println!("3. Bet");
    let mut input1 = String::new();
    io::stdin()
        .read_line(&mut input1)
        .expect("Couldn't read the data!");

    let input: u8 = input1
        .trim()
        .parse()
        .expect("It's not a right number!");

    if input > 4 {
        println!("There are 4 choises.");
        return;
    }
    match input{
        1=>{players.remove("player0");println!("player0 passed");passed=0;},
        2=>println!("player0 checked"),
        3=>{
            println!{"How much?: "}
            let mut betting2 = String::new();
            io::stdin()
            .read_line(&mut betting2)
            .expect("Couldn't read the data!");
            its_raise_time="raise".to_string();
            betting = betting2
            .trim()
            .parse()
            .expect("It's not a right number!");
            if let Some(score) = coins.get_mut("player0") {
            *score -= betting;
            betted+=betting;
            }
        }
        _=>print!("nic")
    }
    let keys_to_process: Vec<String> = players
        .keys()
        .filter(|&k| k != "player0")
        .cloned() // Clone the keys to own them in the vector
        .collect();
    let mut betting1=0;
    for player_number in keys_to_process{

        if let Some(score) = coins.get_mut(&player_number){
             betting1 = rng.gen_range(betting..=betting+*score/2);
        }
        let choice = rng.gen_range(1..=3);
        match choice{
            1=>{println!("{} passed.",player_number);players.remove(&player_number);}
            2=>{
                println!("{} checked",player_number);
                if let Some(coinage) = coins.get_mut(&player_number){
                    *coinage-=betting;
                    betted+=betting;
                }
            }
            3=>{
                betting=betting1;
                if let Some(coinage)=coins.get_mut(&player_number){
                    *coinage-=betting;
                    betted+=betting;
                }
                println!("{} {}ed {}",player_number,its_raise_time,betting);
            }
            _=>println!("nic")
        }

    }
    table.extend(dealing(deck.clone(),0,wich_deal));
     for i in 0..3{
        println!("On the table");
        println!(" value: {} suit: {}, ",table[i].value,table[i].suit);

    }


    for _d in 0..2{
        if passed==1{
        println!("what do you want to do?:");
        println!("1. Pas");
        println!("2. Check");
        println!("3. {}",its_raise_time);
        let mut input1 = String::new();
        io::stdin()
            .read_line(&mut input1)
            .expect("Couldn't read the data!");

        let input: u8 = input1
            .trim()
            .parse()
            .expect("It's not a right number!");

        if input > 4 {
            println!("There are 4 choises.");
            return;
        }
        match input{
            1=>{players.remove("player0");println!("player0 passed");passed=0;},
            2=>println!("player0 checked"),
            3=>{
                println!{"How much?: "}
                let mut betting2 = String::new();
                io::stdin()
                .read_line(&mut betting2)
                .expect("Couldn't read the data!");
                its_raise_time="raise".to_string();
                betting = betting2
                .trim()
                .parse()
                .expect("It's not a right number!");
                if let Some(score) = coins.get_mut("player0") {
                *score -= betting;
                betted+=betting;
                println!("On the line: {}",betted);
                }
            }
            _=>println!("nic")
        }
        }
        let keys_to_process: Vec<String> = players
        .keys()
        .filter(|&k| k != "player0")
        .cloned() // Clone the keys to own them in the vector
        .collect();
    let mut betting1=0;
    for player_number in keys_to_process{
              if let Some(score) = coins.get_mut(&player_number){
             betting1 = rng.gen_range(betting..=betting+*score/2);
        }
        let choice = rng.gen_range(1..=3);
        match choice{
            1=>{println!("{} passed.",player_number);players.remove(&player_number);}
            2=>{
                println!("{} checked",player_number);
                if let Some(coinage) = coins.get_mut(&player_number){
                    *coinage-=betting;
                    betted+=betting;
                }
            }
            3=>{
                betting=betting1;
                if let Some(coinage)=coins.get_mut(&player_number){
                    *coinage-=betting;
                    betted+=betting;
                }
                println!("{} {}ed {}",player_number,its_raise_time,betting);
            }
            _=>print!("nic")

        }

    }
    table.extend(dealing(deck.clone(),0,wich_deal));


    }



    combo(table.clone() ,players,&mut scores,number_of_players);
    let mut max_score=0;
    let mut winners = Vec::new();
    for (player, &score) in &scores {
        if score > max_score {
            max_score = score;
            winners.clear();
            winners.push(player.to_string());
        } else if score == max_score {
            winners.push(player.to_string());
        }
    }

    // Print the result
    if !winners.is_empty() {
        println!("The highest score is {} achieved by: {:?}", max_score, winners);
    } else {
        println!("The HashMap is empty.");
    }

}
