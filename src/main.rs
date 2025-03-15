use rand::seq::SliceRandom;
use std::io;
use std::collections::HashMap;
use std::fmt;
#[derive(Clone)]
#[derive(Debug)]
struct Card{
    value: u8,
    suit: String,
    color: u8
}
impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, self.suit)
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
    for i in 0..playing{
        let player=format!("player{}",i+1);
        let mut on_hand :Vec<Card>=table.clone();
        if let  Some(value) = players.get(&player.to_string()){
            on_hand.extend(value.clone());
        }

    on_hand.sort_by(|a,b| b.value.cmp(&a.value));
    let mut combos : u8=0;
        for i in &on_hand{
        println!("{}: {} {}",player,i.value,i.suit )
        }


    for d in 0..on_hand.len() - 1 {
        if on_hand[d].value == on_hand[d + 1].value {
            combos=1;
        }
        if on_hand[d].value == on_hand[d+1].value{
            for _i in d+1..on_hand.len()-2{
                if on_hand[d+1].value==on_hand[d+2].value{
                    combos=2;
                }
            }
        }
    }
    for d in 0..on_hand.len()-2{
        if on_hand[d].value == on_hand[d+1].value && on_hand[d+1].value == on_hand[d+2].value{
            combos=3;
        }
    }
    for d in 0..on_hand.len()-5{
        if on_hand[d+1].value-1==on_hand[d].value && on_hand[d+2].value-1==on_hand[d+1].value && on_hand[d+3].value-1==on_hand[d+2].value && on_hand[d+4].value-1==on_hand[d+3].value{
            combos=4;
            straight=1;
        }
    }
    for d in 0..on_hand.len()-3{
        if on_hand[d].value==on_hand[d+1].value && on_hand[d+1].value==on_hand[d+2].value&&on_hand[d+2].value==on_hand[d+3].value{
            for i in 0..on_hand.len()-2{
                if on_hand[i].value!=on_hand[d].value || on_hand[i+2].value!=on_hand[d].value{
                    if on_hand[i].value==on_hand[i+1].value{
                        combos=6;
                    }
                }
            }
        }
    }
    for d in 0..on_hand.len()-4{
        if on_hand[d].value==on_hand[d+1].value && on_hand[d+1].value==on_hand[d+2].value && on_hand[d+2].value==on_hand[d+3].value && on_hand[d+3].value==on_hand[d+4].value{
            combos=7;
        }
    }
    on_hand.sort_by(|a,b|a.color.cmp(&b.color));
    for d in 0..on_hand.len()-4{
        if on_hand[d].color==on_hand[d+1].color && on_hand[d+1].color==on_hand[d+2].color && on_hand[d+2].color==on_hand[d+3].color && on_hand[d+3].color==on_hand[d+4].color{
            combos=5;
            flush=1;
        }
    }
    if flush==1 && straight==1{
        combos=8;
    }
    for d in 0..on_hand.len(){
        if on_hand[d].value==13 && straight==1 && flush==1{
            combos=9
        }
    }
    println!("{}",combos);
    scores.entry(player.to_string()).and_modify(|v|*v=*v+combos);
    }
}


fn main(){
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

    shuffle(&mut deck);

    let mut players : HashMap<String,Vec<Card>>= HashMap::new();
    let mut scores : HashMap<String,u8> = HashMap::new();
    let mut wich_deal: usize = 0;
    for i in 0..number_of_players{
        let player_number=format!("player{}",i+1);
        let card=dealing(deck.clone(),i,wich_deal);
        players.entry(player_number.to_string()).or_insert_with(Vec::new).extend(card);
        scores.entry(player_number.to_string()).or_insert(0);
    }
    wich_deal=wich_deal+5;
    for i in 0..number_of_players{
        let player_number=format!("player{}",i+1);
        let card=dealing(deck.clone(),i,wich_deal);
        players.entry(player_number.to_string()).or_insert_with(Vec::new).extend(card);
    }
    wich_deal=wich_deal+5;
    let mut table : Vec<Card>= Vec::new();
    for _i in 0..5{
        table.extend(dealing(deck.clone(),_i,wich_deal));
    }

    println!{"On the table:"}
    for card in &table{
        println!(" value: {} suit: {}, ",card.value,card.suit);
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
