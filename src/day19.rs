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
                .collect::<Vec<_>>()
                .first()
                .cloned();

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

    rules.insert(8, Rule::Branches(vec![vec![42],vec![9991]]));
    rules.insert(9991, Rule::Branches(vec![vec![42,42],vec![9992]]));
    rules.insert(9992, Rule::Branches(vec![vec![42,42,42],vec![9993]]));
    rules.insert(9993, Rule::Branches(vec![vec![42,42,42,42],vec![9994]]));
    rules.insert(9994, Rule::Branches(vec![vec![42,42,42,42,42]]));

    rules.insert(11, Rule::Branches(vec![vec![42,31],vec![9995]]));
    rules.insert(9995, Rule::Branches(vec![vec![42,42,31,31],vec![9996]]));
    rules.insert(9996, Rule::Branches(vec![vec![42,42,42,31,31,31],vec![9997]]));
    rules.insert(9997, Rule::Branches(vec![vec![42,42,42,42,31,31,31,31],vec![9998]]));
    rules.insert(9998, Rule::Branches(vec![vec![42,42,42,42,42,31,31,31,31,31]]));

    rules.insert(8, Rule::Branches(vec![vec![42,8]]));

    p2 =
        messages.iter()
        .map(|m| verify(&rules,m))
        .filter(|r| *r)
        .count();
    
    answer(p1,p2)
}