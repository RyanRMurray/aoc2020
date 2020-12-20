use crate::utils::*;

const RIGHT: [Pt;10] = 
    [(9,1),(9,2),(9,3),(9,4),(9,5),(9,6),(9,7),(9,8),(9,9),(9,10)];

//is g_a | g_b a valid configuration?
fn is_valid_neighbour(g_a:&Grid<bool>, g_b:&Grid<bool>) -> bool{

    RIGHT.iter()
    .fold(true, |res,p|
        res &&
        g_a.at(p) == g_b.at(&p.add((-9,0)))
    )
    
}

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

//rotate to see if valid configuration exists
fn other_can_neighbour(g_a:&Grid<bool>,g_b:&Grid<bool>) -> bool{
    let trans = [rotate_grid,rotate_grid,rotate_grid,rotate_grid,flip_grid,rotate_grid,rotate_grid,rotate_grid,rotate_grid];
    let mut a = g_a.clone();
    let mut b = g_b.clone();


    for a_trans in trans.iter(){
        for b_trans in trans.iter(){
            b = b_trans(&b);
            if is_valid_neighbour(&a,&b){
                return true
            }
        }
        a = a_trans(&a);
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

    let mut neighbours: Vec<i32> = vec![0; pieces.len()];

    for a in 0..pieces.len(){
        for b in a+1..pieces.len(){
            if a != b{
                if other_can_neighbour(&pieces[a], &pieces[b]){
                    neighbours[a] += 1;
                    neighbours[b] += 1;
                }
            }
        }
        println!("done {}",a );
    }

    p1 =
        neighbours.iter()
        .zip(ids.iter())
        .filter(|(ns,_)| **ns == 2)
        .fold(1, |acc,(_,id)| acc * id);
    
    answer(p1,p2)
}