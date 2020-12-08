use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

type Rules<'a> = HashMap<&'a str, Vec<(&'a str,u32)>>;

fn has_child<'a>(r:&'a Rules, ch:&str) -> HashSet<&'a str>{
    r.iter()
    .filter( |(_,q)|
        q.iter()
        .filter( |(c,_)|
            c == &ch
        )
        .count()
        > 0
    )
    .map(|(p,_)| *p)
    .collect()
}

fn bags_in(r:&Rules, b: &str) -> u32{
    match r.get(b){
        None => 0,
        Some(s) =>
            s.iter()
            .fold(0, |acc, (sub_b,amt)|
                acc +
                (amt + bags_in(r, sub_b) * amt)
            )
    }
}

pub fn day07(input:String){
    //big ugly parse
    let re =
        Regex::new(
            r"((?P<desc_l>.+) bags contain|(?P<amt>\d) (?P<desc_r>.*?) (bag|bags)(,|.))+"
        ).unwrap();
    
    let mut rules: Rules = HashMap::new();

    for rule in input.split('\n'){
        let mut cs = re.captures_iter(rule);
        let left = cs.next().unwrap().name("desc_l").unwrap().as_str();

        for right in cs{
            let q = rules.entry(left).or_insert(vec![]);
            q.push(
                (
                    right.name("desc_r").unwrap().as_str()
                ,
                    right.name("amt").unwrap().as_str().parse().expect(":(")
                )
            );
        }
    }

    //part 1: Find reachable from "shiny gold" bag

    let mut reachable: HashSet<&str> = HashSet::new();
    reachable.insert("shiny gold");
    let mut inspect: Vec<&str> = Vec::new();
    inspect.push("shiny gold");

    while inspect.len() > 0{
        let i = inspect.pop().unwrap();
        let new_set = has_child(&rules, i);

        let new_reach: Vec<_> = new_set
            .difference(&reachable)
            .map(|x| *x)
            .collect();

        for x in new_reach{
            inspect.push(x);
        }

        reachable = reachable.union(&new_set).map(|x|*x).collect();
    }

    println!("Part 1: {}", reachable.len() - 1);

    //part 2: find number of bags in shiny gold bag

    println!("Part 2: {}", bags_in(&rules, "shiny gold"))
}