use crate::utils::answer;
use arraydeque::{ArrayDeque, Wrapping};
use std::cmp::Ordering;
use std::collections::VecDeque;

const SIZE: usize = 25;

type Visible = ArrayDeque<[i64;SIZE],Wrapping>;

fn validate(q:&Visible, val:i64) -> bool{
    q.iter()
    .find( |a|
        q.iter()
        .find( |b|
            **b == val - **a
        )
        .is_some()
    )
    .is_some()
}

fn contig_set(mut vals: Vec<i64>, magic_num: i64) -> VecDeque<i64>{
    let mut set: VecDeque<i64> = VecDeque::new();
    let mut sum: i64 = 0;

    while sum != magic_num{
        match sum.cmp(&magic_num){
            Ordering::Greater =>
                sum -= set.pop_front().unwrap(),
            Ordering::Less =>{
                sum += vals[0];
                set.push_back(vals[0]);
                vals.remove(0);
            },
            _ => break
        }
    }
    return set
}

pub fn day09(input:String) -> (String, String){
    let p1;
    let p2;
    let vals = 
        input.lines()
        .map(|x| x.parse().expect(":("))
        .collect::<Vec<i64>>();
    
    //load preamble, create the visible set
    let (pre,queue) = vals.split_at(SIZE);

    let mut visible: Visible = ArrayDeque::new();
    
    for v in pre{
        visible.push_back(*v);
    }

    //part 1: find the first number that cannot be summed by a pair in the visible set
    let mut magic_num = 0;
    for q in queue.into_iter(){
        if !validate(&visible, *q){
            magic_num = *q;
            break;
        }
        visible.push_back(*q);
    }
    
    p1 = magic_num;

    //part 2: find the contiguous set that sums to the magic number

    let mut res =
        contig_set(vals, magic_num)
        .into_iter()
        .collect::<Vec<_>>();

    res.sort();

    p2 = res[0] + res.last().unwrap();

    answer(p1, p2)
}