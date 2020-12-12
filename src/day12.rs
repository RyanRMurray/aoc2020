use crate::utils::*;

pub fn day12(input:String) -> (String,String){
    let p1;
    let p2;
    let dirs: [Pt;4] = [(1,0),(0,1),(-1,0),(0,-1)]; //E S W N
    let mut d_ptr: i32 = 0;

    let mut pos: Pt = (0,0);

    let instrs: Vec<(&str,i32)> = 
        input.lines()
        .map( |l|
            l.split_at(1)
        )
        .map( |(i,v)|
            (i,v.parse::<i32>().expect(":("))
        )
        .collect();

    for (i,v) in instrs.iter(){
        match (i,v){
            (&"F",&v) => pos = pos.add(dirs[d_ptr as usize].mul(v)),
            (&"E",&v) => pos = pos.add(dirs[0].mul(v)),
            (&"S",&v) => pos = pos.add(dirs[1].mul(v)),
            (&"W",&v) => pos = pos.add(dirs[2].mul(v)),
            (&"N",&v) => pos = pos.add(dirs[3].mul(v)),
            (&"R",&v) => d_ptr = cool_mod(d_ptr + (v/90), 4),
            (&"L",&v) => d_ptr = cool_mod(d_ptr - (v/90), 4),
            _ => panic!(":(")
        }
    }

    p1 = pos.mag();
    
    pos = (0,0);
    let mut waypoint = (10,-1);

    for (i,v) in instrs{
        match (i,v){
            ("F",v) => pos = pos.add(waypoint.mul(v)),
            ("E",v) => waypoint = waypoint.add(dirs[0].mul(v)),
            ("S",v) => waypoint = waypoint.add(dirs[1].mul(v)),
            ("W",v) => waypoint = waypoint.add(dirs[2].mul(v)),
            ("N",v) => waypoint = waypoint.add(dirs[3].mul(v)),
            ("R",v) => for _ in 0..(v/90){waypoint = waypoint.rot90cw()},
            ("L",v) => for _ in 0..(v/90){waypoint = waypoint.rot90acw()},
            _ => panic!(":(")
        }
    }

    p2 = pos.mag();

    answer(p1,p2)
}