use crate::utils::answer;
use std::collections::HashSet;

pub fn day06(input:String) -> (String,String){
    let p1;
    let p2;
    //part 1: create sets for each group, then sum their lengths
    let answers =
        input.split("\n\n")
        .map( |g|
            g.chars()
            .filter(|c| *c != '\n')
            .collect::<HashSet<char>>()
        );
    
    p1 = answers.fold(0, |acc,s| acc + s.len());

    //part 2: create sets for each individual, get the intersections of each group, then sum their lengths
    //note to self, 'a'..'z' isn't the alphabet, 'a'..='z' is. 
    let alphabet: HashSet<char> = ('a'..='z').collect();
    let answers2 =
        input.split("\n\n")
        .map( |g|
            g.lines()
            .map( |i|
                i.chars()
                .collect()
            )
            .fold( alphabet.clone(),
                |acc,s| acc.intersection(&s).map(|&x|x).collect()
            )
        );
    
    p2 = answers2.fold(0, |acc,a| acc + a.len());
    answer(p1,p2)
}