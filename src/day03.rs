use std::collections::HashMap;

type Pt = (usize,usize);

fn toboggan((frx,fry): Pt, (tox,toy): Pt, (bx,_): Pt) -> Pt{
    (
        (frx + tox) % (bx),
        fry + toy
    )
}

pub fn day03(input: String){
    //construct grid
    let mut grid: HashMap<Pt, bool> = HashMap::new();
    let bounds;
    let mut ptr = (0,0);
    let mut counter = 0;

    for (i,l) in input.lines().enumerate(){
        for (j,c) in l.chars().enumerate(){
            grid.insert((j,i), c == '#');
        }
    }
    
    match grid.keys().max(){
        Some((a,b)) => {bounds = (a+1,b+1)},
        _       => panic!("No grid detected!")
    }
    
    //part 1: count trees on a slope of (3,1)
    let slope = (3,1);
    while ptr.1 < bounds.1{
        ptr = toboggan(ptr, slope, bounds);

        match grid.get(&ptr){
            Some(true) => counter += 1,
            _          => ()
        }
    }

    println!("Part 1: {}", counter);

    //part 2: count trees on other slopes, find product

    let mut others = [0; 4];
    let slopes = [(1,1),(5,1),(7,1),(1,2)];
    let mut sum: i64 = counter;

    for i in 0..4{
        ptr = (0,0);
        while ptr.1 < bounds.1{
            ptr = toboggan(ptr, slopes[i], bounds);
    
            match grid.get(&ptr){
                Some(true) => others[i] += 1,
                _          => ()
            }
        }
        sum = sum * others[i];
    }

    println!("Part 2: {}", sum);
}