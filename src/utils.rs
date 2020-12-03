#![allow(dead_code)]
use std::collections::HashMap;

pub type Pt = (i32,i32);

#[derive(Default, Debug)]
pub struct Grid<T>{
    map       : HashMap<Pt,T>,
    default   : T,
    pub min_x : i32,
    pub max_x : i32,
    pub min_y : i32,
    pub max_y : i32,
    pub ptr   : Pt
}

impl<T> Grid<T>{
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
}