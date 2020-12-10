use crate::utils::answer;
use std::collections::HashSet;

fn get_id(pass:&str) -> u32{
    return 
        pass.chars()
        .fold(0, |i,c| (i<<1) + ("BR".contains(c) as u32));
}

pub fn day05(input:String) -> (String,String){
    let p1;
    let mut p2 = 0;
    let passes: Vec<&str> = input.lines().collect();

    //get max id
    let ids = 
        passes.iter()
        .map(|s| get_id(s))
        .collect::<Vec<u32>>();
    
    p1 = *ids.iter().max().unwrap();

    //get unused id
    let id_set = ids.into_iter().collect::<HashSet<u32>>();
    let unused = (0..1024).collect::<HashSet<u32>>();

    for u in unused.difference(&id_set){
        if id_set.contains(&(u+1)) && id_set.contains(&(u-1)){
            p2 = *u;
            break;
        }
    }

    answer(p1,p2)
}