use crate::utils::*;

#[derive(Debug,PartialEq,Eq,Clone)]
enum Exprs{
    OpenPar,
    ClosePar,
    Val(u64),
    Mul,
    Add,
    SubExpr(Vec<Exprs>)
}

//there is probably a more in-built way of doing this. In Haskell you'd
//just pass (+), (*) or (id)
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

//evaluate the expression
fn evaluate(mut exs: Vec<Exprs>) -> u64{
    let mut total = 0;
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
            Some(Exprs::SubExpr(sub)) => {
                total = operator(total, evaluate(sub))
            }
            _ => {
                done = true;
            }
        }

    }

    return total
}

//parse sub-expressions (parentheses)
fn parse_subs(mut exs: Vec<Exprs>) -> Vec<Exprs>{
    let mut result: Vec<Exprs> = vec![];

    while exs.len() > 0{
        match exs.pop(){
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
                sub_expr.pop();
                sub_expr.reverse();
                result.push(Exprs::SubExpr(
                    parse_subs(sub_expr)
                ));
            }
            //You may be asking yourself why i'd need to parse for sub-expressions
            //within sub expressions. One word:
            //spaghetti
            Some(Exprs::SubExpr(sub)) => { 
                result.push(Exprs::SubExpr(
                    parse_subs(sub)
                ))
            }
            Some(e) => result.push(e),
            _ => {}
        }
    }
    result.reverse();
    result
}

//turn all additions into sub-expressions so they're evaluated before muls
fn add_precedence(before: Vec<Exprs>) -> Vec<Exprs>{
    let mut after = before.clone();
    let mut i = 0;

    //apply to sub-expressions first
    while i < after.len(){
        match &after[i]{
            Exprs::SubExpr(sub) => {
                after[i] = Exprs::SubExpr(
                    add_precedence(sub.to_vec())
                )
            }
            _ => {}
        }
        i += 1;
    }

    i = 0;

    while i < after.len(){
        match &after[i]{
            Exprs::Add => {
                after[i] = Exprs::SubExpr(
                    vec![
                        after[i-1].clone(),
                        after[i  ].clone(),
                        after[i+1].clone()
                    ]
                );
                after.remove(i-1);
                after.remove(i);
                i -= 1;
            }
            _ => {}
        }
        i += 1
    }

    parse_subs(after)
}

pub fn day18(input:String) -> (String,String){
    let p1: u64;
    let p2: u64;

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
        .map(|l| parse_subs(l))
        .collect();

    //part 1: just evaluate the parsed expressions
    p1 = 
        input_lines.iter()
        .map(|l| evaluate(l.to_vec()))
        .sum();

    //part 2: evaluate after turning adds into sub-expressions
    //to promote their precedence
    p2 =
        input_lines.into_iter()
        .map(|l| 
            evaluate(
                add_precedence(l.to_vec())
            )
        )
        .sum();

    answer(p1,p2)
}