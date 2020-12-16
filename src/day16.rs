use crate::utils::*;
use regex::Regex;
use itertools::Itertools;

#[derive(Debug)]
struct Rule<'a>{
    label : &'a str,
    a_min : i32,
    a_max : i32,
    b_min : i32,
    b_max : i32
}

impl Rule<'_>{
    fn valid(&self, n:i32) -> Option<&str>{

        if self.a_min <= n && self.a_max >= n ||
           self.b_min <= n && self.b_max >= n{
            return Some(self.label)
        }

        None
    }
}

struct RuleSet<'a>{
    rules : Vec<Rule<'a>>
}

impl RuleSet<'_>{
    fn valids(&self, n:i32) -> Vec<&str>{
        self.rules.iter()
        .map   (|r| r.valid(n))
        .filter(|r| r.is_some())
        .map   (|r| r.unwrap())
        .collect()
    }

    fn some_valid(&self, n:i32) -> bool{
        let res = self.valids(n);

        match res.first(){
            Some(_) => true,
            None    => false
        }
    }
}

fn int_from_match(m:Option<regex::Match<'_>>) -> i32{
    m.unwrap().as_str().parse().unwrap()
}


pub fn day16(input:String) -> (String,String){
    let p1: i32;
    let p2: u64;
    let re_rules = Regex::new(
        r"([\w ]+): (\d+)-(\d+) or (\d+)-(\d+)"
    )
    .unwrap();
    let re_numbers = Regex::new(
        r"(\d+)"
    )
    .unwrap();

    //parse input into: rules, my ticket, scanned tickets
    let sections: Vec<&str> =
        input.split("\n\n")
        .collect();

    let rules= RuleSet{rules :
        re_rules.captures_iter(sections[0])
        .map(|c|
            Rule{
                label : c.get(1).unwrap().as_str(),
                a_min : int_from_match(c.get(2)),
                a_max : int_from_match(c.get(3)),
                b_min : int_from_match(c.get(4)),
                b_max : int_from_match(c.get(5)),
            }
        )
        .collect()
    };
    
    //note: u64 owing to very big value for part 2
    let my_ticket: Vec<u64> =
        re_numbers.captures_iter(sections[1])
        .map(|c|
            c.get(0).unwrap().as_str().parse::<u64>().unwrap()
        )
        .collect();
    
    let scanned: Vec<Vec<i32>> =
        sections[2].split('\n')
        .skip(1)
        .map(|l|
            re_numbers.captures_iter(l)
            .map(|c|
                int_from_match(c.get(0))
            )
            .collect()
        )
        .collect();

    //part 1: find all values for which no rule applies
    p1 = 
        scanned.iter()
        .flat_map(|v| v.iter())
        .filter(|n| rules.some_valid(**n))
        .sum();

    //part 2: drop invalid tickets, using valid rules
    let valid_tickets: Vec<Vec<Vec<&str>>> =
        scanned.into_iter()
        .map(|t|
            t.iter()
            .map(|n| rules.valids(*n))
            .collect::<Vec<Vec<&str>>>()
        )
        .filter(|t|
            t.iter()
            .filter(|&vs| vs.len() == 0)
            .count()
            == 0
        )
        .collect();

    //get the categories each index can apply to
    let mut categories: Vec<(usize,Vec<&str>)> = vec![];

    for i in 0..valid_tickets[0].len(){
        categories.push((i,
            valid_tickets.iter()
            .map(|t| t[i].clone())
            .fold1(|acc,next| intersection(acc, next))
            .unwrap()
            )
        )
    }

    categories.sort_by(|(_,a),(_,b)| a.len().cmp(&b.len()));

    let mut taken: Vec<&str> = vec![];
    let mut order: Vec<usize> = vec![];

    //solve which category applies to which index
    for (i,c) in categories{
        taken.push(
            set_subtract(c, &taken)
            .first()
            .unwrap()
        );
        order.push(i)
    }
    
    //get the product of every "departure" field in my ticket
    p2 =
        order.iter().zip(taken)
        .filter(|(_,label)|
            label.contains("departure")
        )
        .map(|(&i,_)|
            my_ticket[i]
        )
        .product();

    answer(p1,p2)
}