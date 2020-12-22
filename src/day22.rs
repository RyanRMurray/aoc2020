use crate::utils::*;
use regex::Regex;
use std::collections::VecDeque;

//returns true on a player 1 win
fn recursive_game(p1_deck: &mut VecDeque<usize>, p2_deck: &mut VecDeque<usize>) -> bool{
    let mut history: Vec<[VecDeque<usize>;2]> = vec![];

    while p1_deck.len() > 0 && p2_deck.len() > 0{
        
        let config = [p1_deck.clone(),p2_deck.clone()];
        if history.contains(&config){
            return true
        }
        history.push(config);

        let a = p1_deck.pop_front().unwrap();
        let b = p2_deck.pop_front().unwrap();

        //if can recurse, do that. else, higher value.
        if a <= p1_deck.len() && b <= p2_deck.len(){
            let mut sub_1 = p1_deck.clone();
            let mut sub_2 = p2_deck.clone();
            sub_1.resize(a, 0);
            sub_2.resize(b, 0);

            if recursive_game(&mut sub_1, &mut sub_2){
                p1_deck.push_back(a);
                p1_deck.push_back(b);
            }else{
                p2_deck.push_back(b);
                p2_deck.push_back(a);
            }
        }else{
            
            if a > b{
                p1_deck.push_back(a);
                p1_deck.push_back(b);
            }else{
                p2_deck.push_back(b);
                p2_deck.push_back(a);
            }
        }
    }
    
    p1_deck.len() > 0
}


pub fn day22(input:String) -> (String,String){
    let p1: usize;
    let p2: usize;
    let re = Regex::new(
        r"Player 1:(?P<p1>(\n\d+)+)\n\nPlayer 2:(?P<p2>(\n\d+)+)"
    ).unwrap();

    //parse: create mutable decks
    let cs = re.captures(&input).unwrap();

    let starting_decks =
        [
            cs.name("p1").unwrap().as_str()
            .split("\n")
            .skip(1)
            .map(|v| v.parse::<usize>().unwrap())
            .collect::<VecDeque<_>>()
        ,
            cs.name("p2").unwrap().as_str()
            .split("\n")
            .skip(1)
            .map(|v| v.parse::<usize>().unwrap())
            .collect::<VecDeque<_>>()
        ];

    let mut decks = starting_decks.clone();

    //part 1: play game until one player has all cards
    while decks[0].len() > 0 && decks[1].len() > 0{
        let a = decks[0].pop_front().unwrap();
        let b = decks[1].pop_front().unwrap();

        if a > b{
            decks[0].push_back(a);
            decks[0].push_back(b);
        }else{
            decks[1].push_back(b);
            decks[1].push_back(a);
        }
    }

    let winner = if decks[0].len() == 0 {&decks[1]} else {&decks[0]};

    p1 = 
        (1..).zip(winner.iter().rev())
        .map(|(n,c)| n * c)
        .sum();

    decks = starting_decks.clone();
    
    let mut p1_deck = decks[0].clone();
    let mut p2_deck = decks[1].clone();

    recursive_game(&mut p1_deck, &mut p2_deck);

    let winner = if p1_deck.len() == 0 {&p2_deck} else {&p1_deck};

    p2 = 
        (1..).zip(winner.iter().rev())
        .map(|(n,c)| n * c)
        .sum();

    answer(p1,p2)
}