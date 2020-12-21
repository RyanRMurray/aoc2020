use crate::utils::*;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

type Food<'a> = (Vec<&'a str>, Vec<&'a str>);
type AllergenMap<'a> = HashMap<&'a str,HashSet<&'a str>>;

fn possibly_contaminated<'a>(m: &AllergenMap<'a>) -> HashSet<&'a str>{
    let mut als: HashSet<&'a str> = HashSet::new(); 
    for vs in m.values(){
        for v in vs{
            als.insert(v);
        }
    }

    als
}

//its been 21 days and i'm still not sure how ownership works.
fn ingr_aller_pairs<'a>(m: &mut AllergenMap<'a>) -> Vec<(&'a str,&'a str)>{
    let mut l = vec![];

    while m.len() > 0{
        for (a,is) in m.clone().iter(){
            if is.len() == 1{
                let i = *is.iter().next().unwrap();
                l.push((i,*a));
                m.remove(a);

                for k in m.clone().keys(){
                    m.get_mut(k).unwrap().remove(i);
                }
            }
        }
    }

    l
}

pub fn day21(input:String) -> (String,String){
    let p1;
    let mut p2 = "".to_owned();
    let re = Regex::new(
        r"(?P<ingrs>.*?) \(contains (?P<allers>(\w+(, )?)+)\)"
    ).unwrap();

    //parse into inputs
    let foods: Vec<Food> =
        re.captures_iter(&input)
        .map( |m|
            (
                m.name("ingrs")
                .unwrap()
                .as_str()
                .split(" ")
                .collect()
            ,
                m.name("allers")
                .unwrap()
                .as_str()
                .split(", ")
                .collect()
            )
        )
        .collect();
    
    //also find which ingredients possible contain a given allergen,
    //and the set of all ingredients
    let mut possible_allergens: AllergenMap = HashMap::new();
    let ingredients: HashSet<&str> =
        foods.iter()
        .map( |(is,_) | is.clone())
        .flatten()
        .collect();

    for (ins,als) in foods.iter(){
        let mut in_set:HashSet<&str> = HashSet::new();
        for i in ins{
            in_set.insert(i);
        }
        for a in als{
            let e = possible_allergens.entry(a).or_insert(ingredients.clone());
            *e = e.intersection(&in_set).map(|i| *i).collect();
        }
    }
    
    //part 1: find count of ingredients that cannot be allergens
    //That is to say, they didn't end up on our 'possible_allergens' map
    let cont = possibly_contaminated(&possible_allergens);

    p1 = 
        foods.iter()
        .map(|(is,_)| 
            is.iter()
            .filter(|i| !cont.contains(*i))
        )
        .flatten()
        .count();
    
    //part 2: find which ingredient contains which allergen, and list them
    //in alphabetical allergen order. Much like the ticket problem, inputs
    //will always allow for each ingredient to be allocated a single allergen
    //in a very easily computable way
    let mut pairs = ingr_aller_pairs(&mut possible_allergens);
    pairs.sort_by(|(_,a),(_,b)| a.cmp(b));

    for (i,_) in pairs.iter(){
        p2.push_str(i);
        p2.push_str(",")
    }

    p2.pop();

    answer(p1,p2)
}