use crate::utils::*;
use std::collections::HashMap;

#[derive(PartialEq,Debug,Clone,Copy,Hash,Eq)]
enum CoOrd{
    //x,y,z
    Pt3(i32,i32,i32),
    //x,y,z,w
    Pt4(i32,i32,i32,i32)
}

type CubeGrid = HashMap<CoOrd,bool>;
type NextGrid = HashMap<CoOrd,u8>;

//Is this hacky? It's probably fine.
fn co_ord_add(x: &CoOrd, y: &CoOrd) -> CoOrd{
    match (x,y){
        (CoOrd::Pt3(a,b,c), CoOrd::Pt3(d,e,f)) =>
            CoOrd::Pt3(a+d,b+e,c+f),
        (CoOrd::Pt4(a,b,c,d), CoOrd::Pt4(e,f,g,h)) =>
            CoOrd::Pt4(a+e,b+f,c+g,d+h),
        _ =>
            panic!("Can only add Pt3s to Pt3s, Pt4s to Pt4s!")
    }
}

//Derive the next grid from the rules and active neighbour counts
fn resolve_grid(before: &CubeGrid, counts: NextGrid) -> CubeGrid{
    counts.keys()
    .map( |k|
        match before.get(k){
            Some(true) =>
                (*k, counts[k] == 2 || counts[k] == 3),
            _ =>
                (*k, counts[k] == 3)
        }
    )
    .collect()
}

//Apply the rules to the input grid given an adjacency list
fn cycle(before: &CubeGrid, adjs: &Vec<CoOrd>) -> CubeGrid{
    let mut counts: NextGrid = HashMap::new();
    let neighbouring_active: Vec<_> = 
        before.iter()
        .map( |(k,v)|
            if *v {
                adjs.iter().map(|a| co_ord_add(k,a)).collect()
            } else {
                vec![]
            }
        )
        .flatten()
        .collect();
    
    for pt in neighbouring_active.into_iter(){
        let c = counts.entry(pt).or_insert(0);
        *c += 1;
    }
    
    resolve_grid(before, counts)
}

//Simply counts the active cubes in a grid
fn count_active(grid: &CubeGrid) -> usize{
    grid.values()
    .filter(|v| **v)
    .count()
}

pub fn day17(input:String) -> (String,String){
    let p1: usize;
    let p2: usize;
    let mut adjacencies_3d: Vec<CoOrd> = vec![];
    let mut adjacencies_4d: Vec<CoOrd> = vec![];

    //very cool nested loop
    //I could technically just enumerate these by hand but i cannot be bothered.
    for x in -1..=1{
        for y in -1..=1{
            for z in -1..=1{
                adjacencies_3d.push(CoOrd::Pt3(x,y,z));
                //WE NEED TO GO DEEPER
                for w in -1..=1{
                    adjacencies_4d.push(CoOrd::Pt4(x,y,z,w));
                }
            }
        }
    }
    //delete (0,0,0), (0,0,0,0). Hard coded because nobody can stop me.
    adjacencies_3d.remove(13);
    adjacencies_4d.remove(40);

    //parse starter grids in 3d and 4d space
    let mut grid_3d: CubeGrid = HashMap::new();
    let mut grid_4d: CubeGrid = HashMap::new();
    let mut y = 0;
    for l in input.lines(){
        let mut x = 0;
        for c in l.chars(){
            grid_3d.insert(CoOrd::Pt3(x,y,0), c == '#');
            grid_4d.insert(CoOrd::Pt4(x,y,0,0), c == '#');
            x += 1;
        }
        y += 1;
    }

    //part 1: six cycles in 3d space
    let after_six_3d =
        (0..6)
        .fold(grid_3d, |g,_|
            cycle(&g, &adjacencies_3d)
        );

    p1 = count_active(&after_six_3d);

    //part 2: six cycles in 4d space
    //re-wrote part 1 solution to use an enum for coordinates, so its just
    //the same function call with different parameters :]]]
    let after_six_4d =
        (0..6)
        .fold(grid_4d, |g,_|
            cycle(&g, &adjacencies_4d)
        );

    p2 = count_active(&after_six_4d);

    answer(p1,p2)
}