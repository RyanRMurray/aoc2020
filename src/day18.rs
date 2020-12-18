use crate::utils::*;

#[derive(Debug,PartialEq,Eq,Clone,Copy)]
enum Exprs{
    OpenPar,
    ClosePar,
    Val(u64),
    Mul,
    Add
}

type Op = fn(u64,u64) -> u64;

fn set(_:u64,b:u64) -> u64{
    b
}

fn add(a:u64,b:u64) -> u64{
    a+b
}

fn mul(a:u64,b:u64) -> u64{
    a*b
}

enum Tree<'a>{
    Leaf(u64),
    Node(&'a Tree<'a>,&'a Tree<'a>,Op)
}

fn to_tree(exs: &mut Vec<Exprs>) -> Tree<'static>{
    let result = Tree::Leaf(0);
    let mut done = false;

    match exs.pop(){
        Exprs::Val(v) => {
            match exs.pop(){
                None => {
                    result = Tree::Leaf(v);
                }
                
            }
        }
    }

    result
}


fn evaluate(mut exs: Vec<Exprs>) -> u64{
    let mut total = 0;
    //true = add, false = mul, none = set
    let mut operator: Op = set;
    let mut done = false;

    while !done{
        match exs.pop(){
            Some(Exprs::Val(v)) => {
                total = operator(total,v);
            }
            Some(Exprs::Mul) => {
                operator = mul;
            }
            Some(Exprs::Add) => {
                operator = add;
            }
            Some(Exprs::OpenPar) => {
                let mut sub_expr: Vec<Exprs> = vec![];
                let mut count = 1;
                let mut ex;
    
                while count != 0{
                    ex = exs.pop().unwrap();
                    match ex{
                        Exprs::OpenPar  =>
                            count += 1,
                        Exprs::ClosePar =>
                            count -= 1,
                        _ => {}
                    }
                    sub_expr.push(ex);
                }
                sub_expr.reverse();
                total = operator(total, evaluate(sub_expr))
            }
            _ => {
                done = true;
            }
        }

    }

    return total
}
pub fn day18(input:String) -> (String,String){
    let p1: u64;
    let p2 = 0;

    //parse to Expressions vector
    let input_lines: Vec<Vec<Exprs>> = 
        input.lines()
        .map(|l|
            l.chars()
            .filter(|c| !c.is_whitespace())
            .map(   |c|
                match c{
                    '(' => Exprs::OpenPar,
                    ')' => Exprs::ClosePar,
                    '*' => Exprs::Mul,
                    '+' => Exprs::Add,
                    oth   => Exprs::Val(oth.to_digit(10).unwrap() as u64)
                }
            )
            .rev()
            .collect()
        )
        .collect();

    p1 = 
        input_lines.iter()
        .map(|l| evaluate(l.to_vec()))
        .sum();

    answer(p1,p2)
}