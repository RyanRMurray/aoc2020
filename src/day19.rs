use crate::utils::*;
use regex::Regex;
use std::collections::HashMap;
use std::str::Chars;
use std::iter::Peekable;

type Sequence = Vec<u32>;
type Parsing<'a> = Option<Peekable<Chars<'a>>>;

#[derive(Debug)]
enum Rule{
    Literal(char),
    Branches(Vec<Sequence>)
}

type RuleSet = HashMap<u32, Rule>;


fn consume<'a>(rules:&RuleSet, rule:u32,input:Parsing<'a>) -> Parsing<'a>{
    if input.is_none(){
        return None
    }


    let mut ins = input.unwrap();

    //println!("Apply {} to: {:?}", rule, ins.clone().collect::<Vec<_>>());

    match &rules[&rule]{
        Rule::Literal(ch) => {
            if ins.peek().is_none(){
                return None
            }
            if ins.peek().unwrap() == ch{
                ins.next();
                return Some(ins)
            }else{
                return None
            }
        }
        Rule::Branches(brs) => {
            let ret = brs.iter()
                .map( |b| 
                    b.iter()
                    .fold(Some(ins.clone()), |q,r| consume(rules, *r,q))
                )
                .filter( |r| r.is_some())
                .nth(0);

            if ret.is_some(){
                return ret.unwrap()
            }
            return None
        }
    }
}

fn verify(rules:&RuleSet, message:&str) -> bool{
    let parsed = consume(rules, 0, Some(message.chars().peekable()));

    parsed.is_some() && parsed.unwrap().count() == 0
}

//part 2 introduces loops in rules 8 and 11, the two components of rule 0.
//Instead of trying to deal with arbitrary loops, we explore repetitions
//up to a fixed number of iterations (6) and see if the message is valid
//assuming that number of iterations. Whinge all you want, it gets results.
fn hacky_part_2_thing(rules:&mut RuleSet, message:&str) -> bool{
    let rule8: Vec<Sequence> = vec![
        vec![42],
        vec![42,42],
        vec![42,42,42],
        vec![42,42,42,42],
        vec![42,42,42,42,42],
        vec![42,42,42,42,42,42],
    ];

    let rule11: Vec<Sequence> = vec![
        vec![42,31],
        vec![42,42,31,31],
        vec![42,42,42,31,31,31],
        vec![42,42,42,42,31,31,31,31],
        vec![42,42,42,42,42,31,31,31,31,31],
        vec![42,42,42,42,42,42,31,31,31,31,31,31],
    ];

    for a in rule8.iter(){
        for b in rule11.iter(){
            let mut c = a.clone();
            c.append(&mut b.clone());
            rules.insert(0, Rule::Branches(vec![c]));

            if verify(rules,message){
                return true
            }
        }
    }

    false
}

pub fn day19(input:String) -> (String,String){
    let p1;
    let p2;
    let re_rules = Regex::new(
        r###"(?P<rnum>\d+):( "(?P<ch>\w)"|(?P<br>(( \d+)+( \|)?)+))"###
    ).unwrap();
    
    let mut ins =
        input.split("\n\n");

    let mut rules: RuleSet = HashMap::new();

    for c in re_rules.captures_iter(ins.next().unwrap()){
        match c.name("ch"){
            Some(ch) => {
                rules.insert(
                    c.name("rnum").unwrap().as_str().parse().unwrap()
                ,   
                    Rule::Literal(ch.as_str().chars().next().unwrap())
                );
            }
            None => {
                let brs: Vec<Sequence> =
                    c.name("br").unwrap().as_str().split('|')
                    .map( |b|
                        b.split_ascii_whitespace()
                        .map(   |n| n.parse().unwrap())
                        .collect()
                    )
                    .collect();
                
                rules.insert(
                    c.name("rnum").unwrap().as_str().parse().unwrap()
                ,
                    Rule::Branches(brs)
                );
            }
        }
    }

    let messages: Vec<&str> =
        ins.next().unwrap()
        .lines()
        .collect();

    p1 =
        messages.iter()
        .map(|m| verify(&rules,m))
        .filter(|r| *r)
        .count();

    p2 =
        messages.iter()
        .map(|m| hacky_part_2_thing(&mut rules,m))
        .filter(|r| *r)
        .count();
    
    answer(p1,p2)
}