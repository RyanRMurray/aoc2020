use crate::utils::*;

fn toboggan_path(max_y: i32, max_x: i32, &(slx,sly):&Pt) -> Vec<Pt>{
    (0..max_y)
    .filter(|y| sly * y < max_y)
    .map(|y| (((slx * y) % max_x), sly * y))
    .collect()
}

pub fn day03(input: String) -> (String,String){
    let p1;
    let p2;
    //construct grid
    let mut g : Grid<bool> = Default::default();
    let slopes = vec![(3,1), (1,1), (5,1), (7,1), (1,2)];

    g.from_input(input, &|c| c == '#', false);

    //get collisions
    //part 1: print collisions for first slope
    let results : Vec<usize> = slopes.iter()
        .map(|s| toboggan_path(&g.max_y+1, &g.max_x+1, s))
        .map(|p| p.iter().filter(|x| *g.at(x)).count())
        .collect();

    p1 = results[0];

    //part 2: print product of all collisions. output is chonky, so i64 is required.
    p2 = results.iter().fold(1, |r,x| r * (*x as i64));
    
    answer(p1, p2)
}