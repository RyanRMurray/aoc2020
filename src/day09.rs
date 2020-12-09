use arraydeque::{ArrayDeque, Wrapping};
use std::cmp::Ordering;

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

fn contig_set(mut vals: Vec<i64>, magic_num: i64) -> Vec<i64>{
    let mut set: Vec<i64>;
    for v in vals.clone().into_iter(){
        set = vec![v];
        vals.remove(0);
        for vs in vals.iter(){
            set.push(*vs);
            match set.iter().sum::<i64>().cmp(&magic_num){
                Ordering::Equal => return set,
                Ordering::Greater => break,
                _ => ()
            }
        }
    }
    panic!("No set found for part 2. Is the input correct?")
}

pub fn day09(input:String){
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

    println!("Part 1: {}", magic_num);

    //part 2: find the contiguous set that sums to the magic number

    let mut res = contig_set(vals, magic_num);
    res.sort();

    println!("Part 2: {:?}", res[0] + res.last().unwrap())
}