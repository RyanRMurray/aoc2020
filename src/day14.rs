use crate::utils::*;
use regex::Regex;
use std::collections::HashMap;

type U36    = [u8;36];
type Memory = HashMap<U36,U36>;

#[derive(PartialEq,Debug,Clone,Copy)]
enum MaskBits{
    X,
    O,
    Z
}

#[derive(Debug)]
enum Instruction{
    Mask(Vec<MaskBits>),
    Write(u64,u64)
}

fn parse_instructions(ins:String) -> Vec<Instruction>{
    let re = Regex::new(
        r"((mask = (?P<mask>[X10]{36}))|(mem\[(?P<addr>\d+)\] = (?P<val>\d+)))"
    )
    .unwrap();

    let mut instructions: Vec<Instruction> = vec![];
    
    for l in ins.lines(){
        let c = re.captures(l).unwrap();
        
        if c.name("mask").is_some(){
            let m: Vec<MaskBits> = 
                c.name("mask").unwrap().as_str().chars()
                .map(|c| 
                    match c{
                        'X' => MaskBits::X,
                        '0' => MaskBits::Z,
                        '1' => MaskBits::O,
                        _   => panic!(":(")
                    }
                )
                .rev()
                .collect();
            
            instructions.push(Instruction::Mask(m));
        }else{
            instructions.push(Instruction::Write
                (
                    c.name("addr").unwrap().as_str().parse::<u64>().unwrap()
                ,
                    c.name("val").unwrap().as_str().parse::<u64>().unwrap()
                )
            )
        }
    }

    instructions
}

fn to_36_bit(val:u64) -> U36{
    let mut res = [0u8;36];

    let b_str = format!("{:b}", val);

    for (i,b) in (0..).zip(b_str.chars().rev()){
        res[i] = b.to_digit(10).unwrap() as u8;
    }

    res
}

fn mask_bit(m:MaskBits, b: u8) -> u8{
    match m{
        MaskBits::X => b,
        MaskBits::Z => 0,
        MaskBits::O => 1,
    }
}

fn with_mask(m:&Instruction, v: U36) -> U36{
    let mut res = [0;36];
    match m{
        Instruction::Mask(mask) => {
            for i in 0..36{
                res[i] = mask_bit(mask[i], v[i])
            }
        }
        _ => panic!(":(")
    }
    res
}

fn memory_mask(ins:&Instruction, mem:U36) -> Vec<U36>{
    let mut flucs: Vec<usize> = vec![];
    let mut base = [0u8;36];

    match ins{
        Instruction::Write(_,_) => panic!(":<"),
        Instruction::Mask(mask) =>{
            
            for (i,b) in (0..).zip(mask){
                match b{
                    MaskBits::Z => base[i] = mem[i],
                    MaskBits::O => base[i] = 1,
                    MaskBits::X => {
                        base[i] = 0;
                        flucs.push(i);
                    }
                }
            }
        }
    }

    let mut addrs = vec![base];    

    let mut to_change: Vec<Vec<usize>> = vec![];

    //funky powerset, very cool
    for f in flucs{
        for ch in to_change.clone(){
            let mut x = ch.clone();
            x.push(f);
            to_change.push(x);
        }
        to_change.push(vec![f]);
    }
    
    for ch in to_change{
        addrs.push(
            ch.iter().fold(base.clone(), |mut acc,i| {acc[*i] = 1; acc})
        )
    }
    addrs
}

fn multi_write(mem: &mut Memory, mask:&Instruction, at:u64, val: u64){
    for to in memory_mask(mask,to_36_bit(at)){
        mem.insert(to,to_36_bit(val));
    }
}

fn from_u36(v:U36) -> u64{
    (0..).zip(v.iter())
    .map(|(i,&n)| 2u64.pow(i) * n as u64)
    .sum()
}

pub fn day14(input:String) -> (String,String){
    let p1: u64;
    let p2: u64;

    //initialise the memory, mask
    let mut memory: Memory = HashMap::new();
    let mut mask = Instruction::Mask(vec![MaskBits::X;36]);

    let instructions = parse_instructions(input);
    
    //part 1: write to addresses with masked values
    for i in instructions.iter(){
        match i{
            Instruction::Mask(new_m) => {
                mask = Instruction::Mask(new_m.clone());
            }
            Instruction::Write(at,val) => {
                memory.insert(to_36_bit(*at), with_mask(&mask,to_36_bit(*val)));
            }
        }
    }

    p1 = memory.values().map(|v| from_u36(*v)).sum();

    //part 2: write values to masked memories
    memory = HashMap::new();
    mask = Instruction::Mask(vec![MaskBits::X;36]);

    for i in instructions{
        match i{
            Instruction::Mask(new_m) => {
                mask = Instruction::Mask(new_m.clone());
            }
            Instruction::Write(at,val) => {
                multi_write(&mut memory, &mask, at, val)
            }
        }
    }

    p2 = memory.values().map(|v| from_u36(*v)).sum();
    answer(p1,p2)
}