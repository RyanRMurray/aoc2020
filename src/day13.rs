use crate::utils::answer;

pub fn day13(input:String) -> (String,String){
    let p1;
    let p2;

    //get the input; the start time and the bus indexes
    let mut input_it = input.lines();
    let my_time: i32 = input_it.next().unwrap().parse().expect(":(");

    let time_table =
        input_it.next().unwrap()
        .split(",");

    //part 1: find the lowest time from start from which a bus will arrive
    
    let (m_i, m_t) = 
        time_table.clone()
        .filter( |&v| v != "x")
        .map   ( | v| v.parse::<i32>().unwrap())
        .map   ( | v| (v, (v - (my_time % v))))
        .fold  ( (0,i32::MAX), |(i,m),(j,v)| if v < m {(j,v)} else {(i,m)});
    
    p1 = m_i * m_t;
    
    //Part 2: enumerate the indexes to get the offset from sequence start time...
    let enumed_table: Vec<(usize,usize)>= 
        time_table
        .enumerate()
        .filter( |(_,s)| *s != "x")
        .map( |(i,s)|
            (i, s.parse::<usize>().unwrap())
        )
        .collect();
    
    //... get big modulo M for Chinese Remainder theorem...
    let big_modulo: usize = 
        enumed_table.iter()
        .map( |(_,v)| v)
        .product();

    //generate values for CRT:
    //(a,v,m) where x = a mod m, v is an index, x is solution mod M
    let crt_vals: Vec<(usize,usize,usize)>=
        enumed_table.iter()
        .map( |&(i,v)| (v-(i % v),v,big_modulo/v))
        .collect();
    
    let mut sum = 0;

    for (i,b,m) in crt_vals{
        for n in 0..{
            //this gets inverse of x in mod m
            if ((m * n) % b) == 1{
                sum += i * m * n;
                break
            }
        }
    }
    
    p2 = sum % big_modulo;
    answer(p1,p2)

}