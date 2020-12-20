use crate::utils::*;

type Trans = (u32,u32);
type EdgeGroup = Vec<(Vec<bool>,Trans)>;

const RIGHT: [Pt;10] = 
    [(9,1),(9,2),(9,3),(9,4),(9,5),(9,6),(9,7),(9,8),(9,9),(9,10)];
const TOP: [Pt;10] =
    [(0,1),(1,1),(2,1),(3,1),(4,1),(5,1),(6,1),(7,1),(8,1),(9,1)];
const LEFT: [Pt;10] = 
    [(0,1),(0,2),(0,3),(0,4),(0,5),(0,6),(0,7),(0,8),(0,9),(0,10)];
const BOT: [Pt;10] =
    [(0,10),(1,10),(2,10),(3,10),(4,10),(5,10),(6,10),(7,10),(8,10),(9,10)];

const SIDES: [[Pt;10];4] = [TOP,RIGHT,BOT,LEFT];

fn flip_grid(g:&Grid<bool>) -> Grid<bool>{
    let mut ng: Grid<bool> = Grid::new();


    let pts: Vec<(Pt,bool)> = 
        g.as_map().into_iter()
        .map(|((x,y),v)| ((9-x,*y),*v)).collect();
    
    ng.update_grid(pts);

    ng
}

fn rotate_grid(g:&Grid<bool>) -> Grid<bool>{
    let mut ng: Grid<bool> = Grid::new();

    let pts: Vec<(Pt,bool)> = 
        g.as_map().into_iter()
        .map(|(p,v)| 
            (p.rot90cw().add((10,1))
            ,*v
            )
        )
        .collect();
    
    ng.update_grid(pts);

    ng
}

fn all_possible_edges(g:&Grid<bool>) -> EdgeGroup{
    //unflipped
    let mut cp = g.clone();
    let mut edges: EdgeGroup = vec![];

    for flips in 0..1{
        for rots in 0..3{
            for side in SIDES.iter(){
                edges.push(
                    (
                        side.iter().map(|p| *cp.at(p)).collect()
                    ,
                        (rots,flips)
                    )
                )
            }
            cp = rotate_grid(&cp)
        }
        cp = flip_grid(&cp)
    }
    edges
}

fn valid_neighbours(a:&EdgeGroup,b:&EdgeGroup) -> bool{
    for (e,_) in a.iter(){
        if b.iter().map(|(edge,_)| edge.clone()).collect::<Vec<Vec<bool>>>().contains(e){
            return true
        }
    }
    false
}

pub fn day20(input:String) -> (String,String){
    let p1;
    let p2 = 0;
    let mut ids: Vec<u64>     = vec![];
    let mut pieces: Vec<Grid<bool>> = vec![];


    //parse: a series of grids with corresponding IDs
    //all grid pieces will have dimensions (0..9)x(0..9)

    for g in input.split("\n\n"){
        ids.push(
            g[5..9].parse::<u64>().unwrap()
        );
        let mut grid:Grid<bool> = Grid::new();
        grid.from_input(g[10..].to_string(), &|c| c == '#', false);
        grid.update_bounds();

        pieces.push(grid);
    }

    //part 1: find the ids of the corner pieces (by finding pieces with two neighbours)
    let mut neighbours: Vec<i32> = vec![0; pieces.len()];

    //precalculate all the possible edges a piece can have...
    let groups: Vec<EdgeGroup> = 
        pieces.iter()
        .map(|g| all_possible_edges(&g))
        .collect();

    //...and record groups that are valid neighbours
    for a in 0..groups.len(){
        for b in a+1..groups.len(){
            if valid_neighbours(&groups[a], &groups[b]){
                neighbours[a] += 1;
                neighbours[b] += 1;
            }
        }
    }

    p1 =
        neighbours.iter()
        .zip(ids.iter())
        .filter(|(ns,_)| **ns == 2)
        .fold(1, |acc,(_,id)| acc * id);
    
    answer(p1,p2)
}