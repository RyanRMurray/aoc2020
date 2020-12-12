use crate::utils::*;

fn rotate90cw((x,y):Pt) -> Pt{
    (-y,x)
}

fn rotate90acw((x,y):Pt) -> Pt{
    (y,-x)
}

pub fn day12(input:String) -> (String,String){
    let p1;
    let p2;
    let dirs = [(1,0),(0,1),(-1,0),(0,-1)]; //E S W N
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
            (&"F",&v) => pos = pt_add(pos,pt_mul(dirs[d_ptr as usize],v)),
            (&"E",&v) => pos = pt_add(pos,pt_mul(dirs[0],v)),
            (&"S",&v) => pos = pt_add(pos,pt_mul(dirs[1],v)),
            (&"W",&v) => pos = pt_add(pos,pt_mul(dirs[2],v)),
            (&"N",&v) => pos = pt_add(pos,pt_mul(dirs[3],v)),
            (&"R",&v) => d_ptr = cool_mod(d_ptr + (v/90), 4),
            (&"L",&v) => d_ptr = cool_mod(d_ptr - (v/90), 4),
            _ => panic!(":(")
        }
    }

    p1 = pt_mag(pos);
    
    pos = (0,0);
    let mut waypoint = (10,-1);

    for (i,v) in instrs{
        print!("at:{:?}, wp:{:?}, instr: {:?}", pos, waypoint, (i,v));
        match (i,v){
            ("F",v) => pos = pt_add(pos,pt_mul(waypoint,v)),
            ("E",v) => waypoint = pt_add(waypoint,pt_mul(dirs[0],v)),
            ("S",v) => waypoint = pt_add(waypoint,pt_mul(dirs[1],v)),
            ("W",v) => waypoint = pt_add(waypoint,pt_mul(dirs[2],v)),
            ("N",v) => waypoint = pt_add(waypoint,pt_mul(dirs[3],v)),
            ("R",v) => for _ in 0..(v/90){waypoint = rotate90cw(waypoint)},
            ("L",v) => for _ in 0..(v/90){waypoint = rotate90acw(waypoint)},
            _ => panic!(":(")
        }
        println!(", to {:?}", pos);
    }

    p2 = pt_mag(pos);

    answer(p1,p2)
}