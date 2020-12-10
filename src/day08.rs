use crate::utils::answer;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug,Clone)]
enum Instr{
    Nop(i32),
    Acc(i32),
    Jmp(i32)
}

#[derive(Debug)]
enum Success{
    Yes(i32),
    No(i32)
}

impl Success{
    fn unwrap(self) -> i32{
        match self{
            Success::No(v)  => v,
            Success::Yes(v) => v
        }
    }
}

#[derive(Debug,Clone)]
struct Handheld{
    ptr : i32,
    acc : i32,
    instrs : HashMap<i32,Instr>,
    history : HashSet<i32>
}

impl Handheld{
    fn run(&mut self){
        self.history.insert(self.ptr);
        
        match &self.instrs[&self.ptr]{
            Instr::Nop(_) =>
                self.ptr += 1,
            Instr::Acc(v) => {
                self.acc += v;
                self.ptr += 1
            },
            Instr::Jmp(v) =>
                self.ptr += v
        }
    }
    
    fn has_loop(&self) -> bool{
        self.history.contains(&self.ptr)
    }

    fn terminated(&self) -> bool{
        self.instrs.len() == self.ptr as usize
    }

    //returns Yes on successful execution
    fn loop_debug(&mut self) -> Success{

        while !self.has_loop() && !self.terminated(){
            self.run();
        }

        if self.terminated(){
            return Success::Yes(self.acc)
        }else{
            return Success::No(self.acc)
        }
    }
}

fn to_instr(x : &str) -> Instr{
    match x.replace(&['+',' '][..],"").split_at(3){
        ("nop",v) => Instr::Nop(v.parse().expect("Failed to read input!")),
        ("acc",v) => Instr::Acc(v.parse().expect("Failed to read input!")),
        ("jmp",v) => Instr::Jmp(v.parse().expect("Failed to read input!")),
        _         => panic!("Unrecognised command!")
    }
}

pub fn day08(input:String) -> (String, String){
    let p1;
    let p2;
    let instrs : HashMap<i32,Instr> =
        (0..).zip(
            input.lines()
            .map(to_instr)
        )
        .collect();
    
    //original machine. Copied for debugging
    let machine = Handheld{
        ptr : 0,
        acc : 0,
        instrs : instrs,
        history : HashSet::new()
    };

    //part 1: run until a loop is found
    p1 = machine.clone().loop_debug().unwrap();

    //part 2: enumerate variants, find acc of terminating

    let targets : Vec<i32> = 
        machine.instrs.iter()
        .filter( |(_,v)|
            match v{
                Instr::Nop(_) => true,
                Instr::Jmp(_) => true,
                _ => false
            }
        )
        .map(|(k,_)| *k)
        .collect();
    
    let variants : Vec<Handheld> =
        targets.iter()
        .map(|k|
            match machine.instrs[k]{
                Instr::Nop(v) => {
                    let mut m = machine.clone();
                    m.instrs.insert(*k, Instr::Jmp(v));
                    m
                },
                Instr::Jmp(v) => {
                    let mut m = machine.clone();
                    m.instrs.insert(*k, Instr::Nop(v));
                    m
                },
                _ => panic!("That shouldn't happen!")
            }
        )
        .collect();
    
    p2 =
        variants.into_iter()
        .map(|mut m| m.loop_debug())
        .find( |res|
            match res{
                Success::No(_) => false,
                Success::Yes(_) => true
            }
        )
        .unwrap()
        .unwrap();
    
    answer(p1, p2)
}