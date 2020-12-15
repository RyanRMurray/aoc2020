use crate::utils::answer;
use std::collections::HashSet;
use std::collections::HashMap;

fn iterate_to(turn_counter: &mut i32, spoken: &mut HashSet<i32>, spoken_last: &mut HashMap<i32,i32>, last_number: &mut i32, limit: i32){

    while *turn_counter < limit{
        if spoken.contains(&last_number){
            let x = *last_number;
            *last_number = *turn_counter - spoken_last[last_number];
            spoken_last.insert(x,*turn_counter);
        }else{
            spoken.insert(*last_number);
            spoken_last.insert(*last_number, *turn_counter);
            *last_number = 0;
        }
        *turn_counter += 1;
    }
}

pub fn day15(input:String) -> (String,String){
    let p1;
    let p2;

    //initialise
    let starting: Vec<i32> =
        input.split(',')
        .map(|v| v.parse().unwrap())
        .collect();

    let mut spoken: HashSet<i32> = HashSet::new();
    let mut spoken_last: HashMap<i32,i32> = HashMap::new();
    let mut turn_counter = 0;
    let mut last_number = 0;

    for s in starting.iter(){
        if s != starting.last().unwrap(){
            turn_counter +=1;
            spoken_last.insert(*s,turn_counter);
            spoken.insert(*s);
        }
        last_number = *s;
    }
    turn_counter +=1;

    //part 1: find 2020th number
    iterate_to(&mut turn_counter, &mut spoken, &mut spoken_last, &mut last_number, 2020);
    p1 = last_number;

    //part 2: find 30000000th number. takes about 4 seconds on a release build
    iterate_to(&mut turn_counter, &mut spoken, &mut spoken_last, &mut last_number, 30000000);
    p2 = last_number;

    answer(p1,p2)
}