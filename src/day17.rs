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

fn cycle(before: &CubeGrid, adjs: &Vec<CoOrd>) -> CubeGrid{
    let mut next: NextGrid = HashMap::new();
    let counts: Vec<_> = 
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
    let mut adjacencies: Vec<CoOrd> = vec![];

    //very cool nested loop
    for x in -1..=1{
        for y in -1..=1{
            for z in -1..=1{
                adjacencies.push(CoOrd::Pt3(x,y,z));
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
            grid.insert(CoOrd::Pt3(x,y,0), c == '#');
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