use std::collections::HashSet;

fn get_id(pass:&str) -> u32{
    let (row,col) = pass.split_at(7);
    let mut rownum = 0;
    let mut colnum = 0;

    for (i,c) in (0..7).rev().zip(row.chars()){
        if c == 'B'{
            rownum += 2u32.pow(i);
        }
    }

    for (i,c) in (0..3).rev().zip(col.chars()){
        if c == 'R'{
            colnum += 2u32.pow(i);
        }
    }

    return (rownum * 8) + colnum
}

pub fn day05(input:String){
    let passes: Vec<&str> = input.lines().collect();

    //get max id
    let ids = 
        passes.iter()
        .map(|s| get_id(s))
        .collect::<Vec<u32>>();
    
    println!("Part 1: {}",
        ids.iter().max().unwrap()
    );
    

    //get unused id
    let id_set = ids.into_iter().collect::<HashSet<u32>>();
    let unused = (0..1024).collect::<HashSet<u32>>();

    for u in unused.difference(&id_set){
        if id_set.contains(&(u+1)) && id_set.contains(&(u-1)){
            println!("Part 2: {}", u);
            break;
        }
    }
}