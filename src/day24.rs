use crate::utils::*;
use regex::Regex;
use std::iter;
use std::collections::HashMap;
use std::collections::HashSet;

const DIRS: [Pt;6] = [(1,0),(0,1),(-1,1),(-1,0),(0,-1),(1,-1)];


pub fn day24(input:String) -> (String,String){
    let p1;
    let p2;
    let re = Regex::new(
        r"(?P<coord>e|se|sw|w|nw|ne)"
    ).unwrap();
    let dirs: HashMap<&str,Pt> =
        ["e","se","sw","w","nw","ne"].iter()
        .zip(DIRS.iter())
        .map(|(k,v)| (*k,*v))
        .collect();

    //parse into paths
    let paths: Vec<Vec<Pt>> = 
        input.lines()
        .map( |l| 
            re.captures_iter(l)
            .map( |c|
                dirs[&c["coord"]]
            )
            .collect()
        )
        .collect();

    //part 1: find flipped from paths
    let mut flipped: HashSet<Pt> = HashSet::new();

    for p in paths{
        let to_flip =
            p.iter()
            .fold((0,0), |acc,to| acc.add(*to));

        if flipped.contains(&to_flip){
            flipped.remove(&to_flip);
        } else {
            flipped.insert(to_flip);
        }
    }

    p1 = flipped.len();

    //part 2: game of life 3: hexagonic edition
    for _ in 0..100{
        let mut next_flipped: HashSet<Pt> = HashSet::new();
        let mut adjs_list: HashMap<Pt,u8> = 
            flipped.iter()
            .map(|(k,v)| (*k,*v))
            .zip(iter::repeat(0))
            .collect();

        for at in &flipped{
            for n in DIRS.iter(){
                *adjs_list.entry(at.add(*n)).or_insert(0) += 1;
            }
        }

        for (at,adjs) in adjs_list{
            if flipped.contains(&at){
                if adjs > 0 && adjs < 3{
                    next_flipped.insert(at);
                }
            } else {
                if adjs == 2{
                    next_flipped.insert(at);
                }
            }
        }
        flipped = next_flipped;
    }

    p2 = flipped.len();
    answer(p1,p2)
}