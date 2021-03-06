#![allow(dead_code)]
use std::collections::HashMap;
use std::collections::hash_map::{Keys,Entry};
use std::fmt::Display;
pub fn intersection<T: PartialEq>(a:Vec<T>, b:Vec<T>) -> Vec<T>{
    b.into_iter()
    .filter( |b_elem|
        a.contains(b_elem)
    )
    .collect()
}

pub fn set_subtract<T: PartialEq>(a:Vec<T>, b:&Vec<T>) -> Vec<T>{
    a.into_iter()
    .filter( |a_elem|
        !b.contains(a_elem)
    )
    .collect()
}

pub fn cool_mod(v:i32, m:i32) -> i32{
    ((v % m) + m) % m
}

pub trait Point<T>{
    fn add(self,other:Pt) -> Self;
    fn mul(self,v:i32)-> Self;
    fn rot90cw(self) -> Self;
    fn rot90acw(self) -> Self;
    fn mag(self) -> i32;
    fn neighbours_4(&self) -> Vec<Pt>;
}

pub type Pt = (i32,i32);

impl Point<Pt> for Pt{
    fn add(self, (ox,oy):Pt) -> Self{
        let (x,y) = self;
        (x+ox,y+oy)
    }
    fn mul(self, v: i32) -> Self{
        let (x,y) = self;
        (x*v,y*v)
    }
    fn mag(self) -> i32{
        let (x,y) = self;
        (x + y).abs()
    }
    fn rot90cw(self) -> Self{
        let (x,y) = self;
        (-y,x)
    }
    fn rot90acw(self) -> Self{
        let (x,y) = self;
        (y,-x)
    }
    fn neighbours_4(&self) -> Vec<Pt>{
        vec![(0,-1),(1,0),(0,1),(-1,0)].iter()
        .map(|n| self.add(*n))
        .collect()
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Grid<T: Default + PartialEq>{
    map       : HashMap<Pt,T>,
    default   : T,
    pub min_x : i32,
    pub max_x : i32,
    pub min_y : i32,
    pub max_y : i32,
    pub ptr   : Pt
}

impl<T: Default + PartialEq> Grid<T>{
    pub fn new() -> Grid<T>{
        Grid{
            map     : HashMap::new(),
            default : T::default(),
            min_x   : 0,
            max_x   : 0,
            min_y   : 0,
            max_y   : 0,
            ptr     : (0,0)
        }
    }

    //takes the input string, some conversion function, and the default value
    pub fn from_input(&mut self, s: String, conv : &dyn Fn(char) -> T, d: T){
        self.default = d;

        for (i,l) in (0..).zip(s.lines()){
            for (j,c) in (0..).zip(l.chars()){
                self.map.insert((j,i), conv(c));
            }
        }

        match self.map.keys().max(){
            Some((x,y)) => {self.max_x = *x; self.max_y = *y;},
            _           => panic!("No grid detected!")
        }
    }

    pub fn at(&self, &p : &Pt) -> &T{
        match self.map.get(&p) {
            Some(r) => r,
            _       => &self.default
        }
    }

    pub fn keys(&self) -> Keys<Pt,T>{
        self.map.keys()
    }

    //get neighbours within the specified bounds
    pub fn neighbours_8(&self, &(x,y): &Pt) -> Vec<Pt>{
        let adjs = [(0,-1), (1,-1), (1,0), (1,1), (0,1), (-1,1), (-1,0), (-1,-1)];

        adjs.iter()
        .map( |(xd, yd)| (x+xd,y+yd))
        .filter( |p|
            self.map.contains_key(p)
        )
        .collect()
    }


    //Now that's what I call Spaghetti™!
    pub fn radial_neighbours_8(&self, &(x,y): &Pt) -> Vec<Pt>{
        let adjs = [(0,-1), (1,-1), (1,0), (1,1), (0,1), (-1,1), (-1,0), (-1,-1)];
        let mut res = vec![];

        for angle in adjs.iter(){
            let (s_x,s_y) = angle;
            let mut a_x   = 0 + s_x;
            let mut a_y   = 0 + s_y;

            while (y+a_y <= self.max_y && y+a_y >= self.min_y) &&
                  (x+a_x <= self.max_x && x+a_x >= self.min_x){
                
                if *self.at(&(x+a_x,y+a_y)) != self.default{
                    res.push((x+a_x,y+a_y));
                    break
                }
                a_x += s_x;
                a_y += s_y;
            }
        }
        res
    }

    //tool for updating a grid with an update queue
    pub fn update_grid(&mut self, ups: Vec<(Pt,T)>){
        for (p,v) in ups{
            self.map.insert(p,v);
        }
    }
    
    pub fn insert(&mut self, p:Pt, val:T){
        self.map.insert(p, val);
    }

    pub fn delete(mut self, at:&Pt) -> Self{
        self.map.remove(at);
        self
    }

    pub fn update_bounds(&mut self){
        match self.map.keys().max(){
            Some((x,y)) => {self.max_x = *x; self.max_y = *y;},
            _           => panic!("No grid detected!")
        }
        match self.map.keys().min(){
            Some((x,y)) => {self.min_x = *x; self.min_y = *y;},
            _           => panic!("No grid detected!")
        }
    }

    pub fn as_map(&self) -> &HashMap<Pt,T>{
        &self.map
    }

    pub fn entry(&mut self, p: Pt) -> Entry<Pt,T>{
        self.map.entry(p)
    }

    pub fn offset(mut self, off:&Pt){
        for (k,_) in self.map.iter_mut(){
            k.add(*off);
        }
    }
}

pub fn answer<A:Display,B:Display>(a:A,b:B) -> (String, String){
    (a.to_string(), b.to_string())
}
