use crate::utils::*;
use arraydeque::{ArrayDeque, Wrapping};

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

#[derive(Clone,Debug)]
struct PuzzlePiece{
    id:usize,
    content:Grid<bool>,
    //TOP,RIGHT,BOTTOM,LEFT
    neighbours: ArrayDeque<[Option<usize>;4],Wrapping>,
    offset:(i32,i32),
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
            neighbours: ArrayDeque::from(vec![None,None,None,None]),
            offset: (0,0),
            sides: s
        }
    }

    fn rotate(&mut self){
        self.content = rotate_grid(&self.content);
        let x = self.neighbours.pop_back().unwrap();
        self.neighbours.push_front(x);
        let x = self.sides.pop().unwrap();
        self.sides.insert(0, x);
    }

    fn flip(&mut self){
        self.content = flip_grid(&self.content);
        self.neighbours.swap(1, 3);
        self.sides.swap(1,3);
        self.sides.get_mut(0).unwrap().reverse();
        self.sides.get_mut(2).unwrap().reverse();
    }

    fn offset_by(mut self, off:&Pt){
        self.offset = *off;
        self.content.offset(off);
    }

    fn to_attachable(mut self){
        self.content =
            SIDES.iter().flatten()
            .fold(self.content, |acc,p| acc.delete(p));
        self.content.offset(&(-1,-1));
    }

    fn n_count(&self) -> usize{
        self.neighbours.iter()
        .filter(|n| n.is_some())
        .count()
    }

    fn try_neighbour(&mut self, other:&mut PuzzlePiece) -> bool{
        let ts = [true,true,true,false,true,true,true,true];

        //for each of my sides, 
        //check if the rotating piece can be attached by its opposite side
        for s in 0..4{
            match self.neighbours[s]{
                Some(_) => {continue;}
                _ => {}
            }

            for t in ts.iter(){

                if self.sides.get(s).unwrap() == other.sides.get((s+2)%4).unwrap() {
                    self.neighbours[s]    = Some(other.id);
                    other.neighbours[(s+2)%4] = Some(self.id);
                    return true
                }else{
                    if *t {other.rotate();} else {other.flip();};
                }
            }
        }
        false
    }

}

pub fn day20(input:String) -> (String,String){
    let p1: usize;
    let p2 = 0;
    let mut ids: Vec<u64> = vec![];
    let mut pieces: Vec<PuzzlePiece> = vec![];

    for (i,g) in (0..).zip(input.split("\n\n")){
        ids.push(
            g[5..9].parse::<u64>().unwrap()
        );
        let mut grid: Grid<bool> = Grid::new();
        grid.from_input(g[10..].to_string(), &|c|c=='#',false);
        grid.update_bounds();
        pieces.push(PuzzlePiece::new(grid,i));
    }
    
    for i in 0..pieces.len(){
        println!("{:?}", i);
        let mut this_piece = pieces.get(i).unwrap().clone();
        for j in i+1..pieces.len(){
            let mut that_piece = pieces.get(j).unwrap().clone();
            if this_piece.try_neighbour(&mut that_piece){
                pieces[i] = this_piece.clone();
                pieces[j] = that_piece.clone();
            }
        }
    }

    p1 =
        pieces.iter()
        .filter(|p| p.n_count() == 2)
        /*.map(|p|
            ids[p.id]
        )*/
        .count();

    answer(p1,p2)
}