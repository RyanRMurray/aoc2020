use crate::utils::*;
use std::collections::HashSet;

#[derive(Debug,PartialEq,Clone)]
enum Space{
    Floor,
    Chair,
    Occupied
}

impl ToString for Space{
    fn to_string(&self) -> String{
        match self{
            Space::Floor    => ".",
            Space::Chair    => "L",
            Space::Occupied => "#"
        }
        .to_string()
    }
}

impl Default for Space{
    fn default() -> Self {Space::Floor}
}

fn to_space(c:&char) -> Space{
    match c{
        '.' => Space::Floor,
        '#' => Space::Occupied,
        'L' => Space::Chair,
        _   => panic!("Unrecognised char in input!")
    }
}

fn update_for(g:&Grid<Space>, p: &Pt, radial: bool) -> Option<Space>{
    if *g.at(p) == Space::Floor{
        return None;
    }

    let it =
        if radial{
            g.radial_neighbours_8(p)
        }else{
            g.neighbours_8(p)
        };

    let occ_count = 
        it.iter()
        .map   (|pos| g.at(pos))
        .filter(|t| **t == Space::Occupied)
        .count();

    match (g.at(p),radial){
        (Space::Chair,_) => {
            if occ_count == 0 {Some(Space::Occupied)} else {None}
        }
        (Space::Occupied,false) => {
            if occ_count  > 3 {Some(Space::Chair)} else {None}
        }
        (Space::Occupied,true) => {
            if occ_count  > 4 {Some(Space::Chair)} else {None}
        }
        _ => None
    }
}

fn step( grid: &mut Grid<Space>, radial:bool) -> usize{
    let mut updates: Vec<(Pt,Space)> = vec![];

    for p in grid.keys(){
        match update_for(&grid,p,radial){
            Some(s) => updates.push((*p,s)),
            _       => ()
        }
    }

    grid.update_grid(updates);

    grid.keys()
    .map   (|k| grid.at(k))
    .filter(|v| **v == Space::Occupied)
    .count()
}

pub fn day11(input:String) -> (String, String){
    let p1;
    let p2;

    //input as a map of enumerated type Space
    let mut grid: Grid<Space> = Grid::new();
    
    grid.from_input(
        input,
        &|c| to_space(&c),
        Space::Floor
    );

    //with a copy for later
    let mut grid2 = grid.clone();

    //part 1, step with rules til equilibrium
    let mut seen: HashSet<usize> = HashSet::new();

    loop{
        let next = step(&mut grid, false);
        if seen.contains(&next){
            p1 = next;
            break
        }else{
            seen.insert(next);
        }
    }

    let mut seen2: HashSet<usize> = HashSet::new();
    
    loop{
        let next = step(&mut grid2,true);
        if seen2.contains(&next){
            p2 = next;
            break
        }else{
            seen2.insert(next);
        }
    }

    answer(p1,p2)
}