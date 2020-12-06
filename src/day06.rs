use std::collections::HashSet;

pub fn day06(input:String){
    //part 1: create sets for each group, then sum their lengths
    let answers: Vec<HashSet<char>> =
        input.split("\n\n")
        .map( |g|
            g.chars()
            .filter(|c| *c != '\n')
            .collect::<HashSet<char>>()
        )
        .collect();

    println!("Part 1: {}", answers.iter().fold(0, |acc,s| acc + s.len()));

    //part 2: create sets for each individual, get the intersections of each group, then sum their lengths
    //note to self, 'a'..'z' isn't the alphabet, 'a'..='z' is. 
    let alphabet: HashSet<char> = ('a'..='z').collect();
    let answers2: Vec<HashSet<char>> =
        input.split("\n\n")
        .map( |g|
            g.lines()
            .map( |i|
                i.chars()
                .collect::<HashSet<char>>()
            )
            .fold( alphabet.clone(),
                |acc,s| acc.intersection(&s).map(|&x|x).collect()
            )
        )
        .collect();
    
    println!("Part 2: {}", answers2.into_iter().fold(0, |acc,a| acc+a.len()))
    
}