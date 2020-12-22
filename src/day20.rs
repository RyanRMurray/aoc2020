use crate::utils::*;
use std::collections::HashSet;

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
    ng.update_bounds();

    ng
}

fn rotate_grid(g:&Grid<bool>, offset: &Pt) -> Grid<bool>{
    let mut ng: Grid<bool> = Grid::new();

    let pts: Vec<(Pt,bool)> = 
        g.as_map().into_iter()
        .map(|(p,v)| 
            (p.rot90cw().add(*offset)
            ,*v
            )
        )
        .collect();
    
    ng.update_grid(pts);
    ng.update_bounds();

    ng
}

#[derive(Clone,Debug,PartialEq)]
struct PuzzlePiece{
    id:usize,
    content:Grid<bool>,
    offset:(i32,i32),
    //TOP,RIGHT,BOTTOM,LEFT
    sides: Vec<Vec<bool>>
}

impl PuzzlePiece{
    pub fn new(g:Grid<bool>, n:usize) -> Self{
        let s = 
            SIDES.iter()
            .map(|sd|
                sd.iter()
                .map(|p| *g.at(&p))
                .collect()
            )
            .collect();

        Self{
            id: n,
            content: g,
            offset: (0,0),
            sides: s
        }
    }

    fn rotate(&mut self){
        self.content = rotate_grid(&self.content, &(10,1));
        let x = self.sides.pop().unwrap();
        self.sides.insert(0, x);
        let t = self.sides.get_mut(0).unwrap();
        t.reverse();
        let b = self.sides.get_mut(2).unwrap();
        b.reverse();
    }

    fn flip(&mut self){
        self.content = flip_grid(&self.content);
        self.sides.swap(1,3);
        let t = self.sides.get_mut(0).unwrap();
        t.reverse();
        let b = self.sides.get_mut(2).unwrap();
        b.reverse();
    }

    fn contents(self) -> Vec<(Pt,bool)>{
        self.content.as_map().iter().map(|(k,v)| (*k,*v)).collect()
    }
}

#[derive(PartialEq,Debug,Clone)]
enum Placement{
    Criteria(Option<Vec<bool>>,Option<Vec<bool>>,Option<Vec<bool>>,Option<Vec<bool>>),
    Piece(PuzzlePiece)
}

impl Default for Placement{
    fn default() -> Placement{
        Placement::Criteria(None,None,None,None)
    }
}


fn fits_criteria(c:&Placement, p:&PuzzlePiece) -> bool{
    match c{
        Placement::Criteria(t,r,b,l) => {
            p.sides.iter().zip(vec![t,r,b,l].iter())
            .all(|(s,c)| match c{None => true, Some(seq) => seq == s})
        }
        _ => false
    }
}

type MasterGrid = Grid<Placement>;

fn fuse_criteria(a:&mut Placement,b:&Placement){
    match (a,b){
        (Placement::Criteria(a,b,c,d),Placement::Criteria(e,f,g,h)) => {
            if a.is_none() {*a = e.clone()}
            if b.is_none() {*b = f.clone()}
            if c.is_none() {*c = g.clone()}
            if d.is_none() {*d = h.clone()}
        }
        _ => {}
    }
}

fn place(g:&mut MasterGrid, piece: PuzzlePiece, pt: Pt){
    let north = g.entry(pt.add((0,-1))).or_insert(Placement::default());
    fuse_criteria(north, &Placement::Criteria(None,None,Some(piece.sides.get(0).unwrap().clone()),None));
    
    let east  = g.entry(pt.add((1,0))).or_insert(Placement::default());
    fuse_criteria(east, &Placement::Criteria(None,None,None,Some(piece.sides.get(1).unwrap().clone())));
    
    let south = g.entry(pt.add((0,1))).or_insert(Placement::default());
    fuse_criteria(south, &Placement::Criteria(Some(piece.sides.get(2).unwrap().clone()),None,None,None));
    
    let west  = g.entry(pt.add((-1,0))).or_insert(Placement::default());
    fuse_criteria(west, &Placement::Criteria(None,Some(piece.sides.get(3).unwrap().clone()),None,None));
    
    g.insert(pt, Placement::Piece(piece));
}

fn fits(g:&MasterGrid, pt:&Pt, piece: &mut PuzzlePiece) -> bool{
    let ts = [true,true,true,false,true,true,true,false];

    for t in ts.iter(){
        if fits_criteria(g.at(pt), &piece){
            return true
        }
        if *t {piece.rotate()} else {piece.flip()}
    }

    false
}

fn identify_corners(g:&MasterGrid) -> Vec<usize>{
    g.keys()
    .filter(|k| 
        match g.at(k){
            Placement::Piece(_) => true, _ => false
        }
    )
    .filter(|k|
        k.neighbours_4().iter()
        .filter(|k| 
            match g.at(k){
                Placement::Piece(_) => true, _ => false
            }
        )
        .count()
        == 2
    )
    .map(|k| match g.at(k){Placement::Piece(p) => p.id, _ => panic!(":(")})
    .collect()
}

fn generate_full_grid(g:MasterGrid) -> Grid<bool>{
    let edge_points: Vec<Pt> = SIDES.iter().map(|s| s.iter()).flatten().map(|v|*v).collect();
    let mut result: Grid<bool> = Grid::new();

    let mut offy = -1;
    for y in g.min_y..=g.max_y{
        let mut offx = -9;
        for x in g.min_x..=g.max_x{
            match g.at(&(x,y)){
                Placement::Piece(p) => {
                    result.update_grid(
                        p.clone().contents()
                        .iter()
                        .filter(|(at,_)| !edge_points.contains(at))
                        .map(|(at,v)| (at.add((offx-1,offy-1)),*v))
                        .collect::<Vec<(Pt,bool)>>()
                    );
                }
                _=>{}
            }
            offx += 9;
        }
        offy += 8;
    }
    result.update_bounds();

    result
}

fn count_monsters(mut g: Grid<bool>) -> usize{
    let ts = [true,true,true,false,true,true,true,false];
    //one wiggly boi
    let nessie = vec![
                                                                                 (18,0),
    (0,1),            (5,1),(6,1),             (11,1),(12,1),             (17,1),(18,1),(19,1),
          (1,2),(4,2),            (7,2),(10,2),              (13,2),(16,2)  
    ];
    let mut monster_count = 0;

    for t in ts.iter(){
        test(&g);
        for y in g.min_y..=g.max_y{
            for x in g.min_x..g.max_x{
                if nessie.iter().all(|off| *g.at(&(x,y).add(*off))){
                    monster_count += 1;
                }
            }
        }

        if monster_count > 0{
            break;
        }
        if *t {g = rotate_grid(&g, &(g.max_y,0)); println!("fleep");} else {g = flip_grid(&g);}
    }
    println!("{}", monster_count);

    monster_count
}

fn test(g:&Grid<bool>){
    for y in g.min_y..=g.max_y{
        for x in g.min_x..=g.max_x{
            print!("{}", if *g.at(&(x,y)) {'#'} else {'.'})
        }
        print!("\n");
    }
    print!("\n");
}

pub fn day20(input:String) -> (String,String){
    let p1: u64;
    let p2;
    let mut ids: Vec<u64> = vec![];
    let mut pieces: Vec<PuzzlePiece> = vec![];
    let mut master: MasterGrid = Grid::new();

    //parse pieces
    for (i,g) in (0..).zip(input.split("\n\n")){
        ids.push(
            g[5..9].parse::<u64>().unwrap()
        );
        let mut grid: Grid<bool> = Grid::new();
        grid.from_input(g[10..].to_string(), &|c|c=='#',false);
        grid.update_bounds();
        pieces.push(PuzzlePiece::new(grid,i));
    }

    //construct full grid
    place(&mut master, pieces.remove(0), (0,0));
    let mut found: HashSet<Pt> = HashSet::new();
    found.insert((0,0));
    let mut to_find: Vec<Pt> = (0,0).neighbours_4();
    let mut subject = (0,0);

    while to_find.len() > 0{
        while found.contains(&subject){
            if to_find.len() == 0{
                break;
            }
            subject = to_find.remove(0);
        }

        let mut candidate: Option<PuzzlePiece> = None;
        for (i, mut p) in (0..).zip(pieces.clone()){
            if fits(&master, &subject, &mut p){
                candidate = Some(p);
                pieces.remove(i);
                break;
            }
        }
        match candidate{
            None => {},
            Some(c) => {
                place(&mut master, c, subject);
                to_find.append(&mut subject.neighbours_4());
            }
        }
        found.insert(subject);
    }
    master.update_bounds();

    //part 1; find product of edge IDs
    p1 = identify_corners(&master).iter()
        .map(|i| ids[*i])
        .product();

    //part 2: construct full grid...

    let res = generate_full_grid(master);
    let dark_spots = res.as_map().values().filter(|v| **v).count();

    p2 = dark_spots - (count_monsters(res) * 15);

    answer(p1,p2)
}