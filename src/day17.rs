use crate::utils::*;
use std::collections::HashMap;

//x,y,z
type Pt3 = (i32,i32,i32);
//x,y,z,w
type Pt4 = (i32,i32,i32,i32);

type CubeGrid = HashMap<Pt3,bool>;
type NextGrid = HashMap<Pt3,u8>;

fn pt3_add((a,b,c):&Pt3,(d,e,f):&Pt3) -> Pt3{
    (a+d,b+e,c+f)
}

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

fn cycle(before: &CubeGrid, adjs: &Vec<Pt3>) -> CubeGrid{
    let mut next: NextGrid = HashMap::new();
    let counts: Vec<_> = 
        before.iter()
        .map( |(k,v)|
            if *v {
                adjs.iter().map(|a| pt3_add(k,a)).collect()
            } else {
                vec![]
            }
        )
        .flatten()
        .collect();
    
    for pt in counts.into_iter(){
        let c = next.entry(pt).or_insert(0);
        *c += 1;
    }
    
    resolve_grid(before, next)
}

fn count_active(grid: &CubeGrid) -> usize{
    grid.values()
    .filter(|v| **v)
    .count()
}

pub fn day17(input:String) -> (String,String){
    let p1: usize;
    let p2 = 0;
    let mut adjacencies: Vec<Pt3> = vec![];

    //very cool nested loop
    for x in -1..=1{
        for y in -1..=1{
            for z in -1..=1{
                adjacencies.push((x,y,z));
            }
        }
    }
    //delete (0,0,0)
    adjacencies.remove(13);

    //parse starter grid
    let mut grid: CubeGrid = HashMap::new();
    let mut y = 0;
    for l in input.lines(){
        let mut x = 0;
        for c in l.chars(){
            grid.insert((x,y,0), c == '#');
            x += 1;
        }
        y += 1;
    }

    let after_six =
        (0..6)
        .fold(grid, |g,_|
            cycle(&g, &adjacencies)
        );

    p1 = count_active(&after_six);

    answer(p1,p2)
}