use crate::utils::*;
use std::char::from_digit;
use std::collections::VecDeque;

type Cups = VecDeque<usize>;

fn play_game(mut cups: Cups, rounds: usize, max:usize) ->  Cups{
    let mut buffer: Vec<usize> = vec![0];
    
    for _ in 0..rounds{

        //take cups to right
        for _ in 0..3{
            buffer.push(cups.remove(1).unwrap());
        }
        
        //get destination
        let mut target = cups[0] - 1;

        while buffer.contains(&target){
            if target < 2{
                target = max
            } else {
                target -= 1
            }
        }
        let dest = cups.iter().position(|&v| v == target).unwrap() + 1;

        for _ in 0..3{
            let x = buffer.pop().unwrap();
            cups.insert(dest, x);
        }

        cups.rotate_left(1);
    }

    cups
}


pub fn day23(input:String) -> (String,String){
    let mut p1 = "".to_string();
    let p2;

    //parse list of integers into the cups
    let ins: Vec<usize> = 
        input.chars()
        .map(|v| v.to_digit(10).unwrap() as usize)
        .collect();

    //part 1: apply rules to just input cups 100 times
    let mut cups: VecDeque<usize> =
        ins.clone().into_iter().collect();

    let m = cups.iter().max().unwrap();

    let p1_res = play_game(cups.clone(), 100, *m);

    let x = p1_res.iter().position(|&v| v == 1).unwrap();

    for i in x+1..cups.len(){
        p1.push(from_digit(p1_res[i] as u32,10).unwrap())
    }

    for i in 0..x{
        p1.push(from_digit(p1_res[i] as u32,10).unwrap())
    }

    //part 2: append cups til we reach 1000,000, then
    //apply rules to those cups 10,000,000 times

    for i in m+1..=1_000_000{
        cups.push_back(i);
    }
    
    let p2_res = play_game(cups, 10_000_000, 1_000_000);
    let x = p2_res.iter().position(|&v| v == 1).unwrap();

    p2 = p2_res[x+1] * p2_res[x+2];

    answer(p1,p2)
}