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

    //part 1: find the lowest n st n > start time where n is the arrival of a bus
    let mut departures: Vec<(i32,i32)> =
        time_table
        .clone()
        .filter(|&v| v != "x")
        .map( |v|
            v.parse().expect(":(")
        )
        .map( |i|
            (i,i)
        )
        .collect();
    
    let under = &|vec:&Vec<(i32,i32)>| 
        vec.iter()
        .filter(|&&(_,v)| v < my_time)
        .count();

    //collect all the minimum values for arrival times greater than start time...
    while under(&departures) > 0{
        departures =
            departures
            .into_iter()
            .map( |(i,t)|
                if t < my_time {(i,i+t)} else {(i,t)}
            )
            .collect();
    }

    //... picking the smallest value
    let (m_i,m_t) =
        departures.iter()
        .fold( (0,i32::MAX), |min, pair|
            if pair.1 < min.1 {*pair} else {min}
        );
    
    p1 = m_i * (m_t - my_time);
    
    //enumerate the indexes to get the offset from start time
    let enumed_table: Vec<(usize,usize)>= 
        time_table
        .enumerate()
        .filter( |(_,s)| *s != "x")
        .map( |(i,s)|
            (i, s.parse::<usize>().unwrap())
        )
        .collect();
    
    //get big modulo for Chinese Remainder theorem
    let big_modulo: usize = 
        enumed_table.iter()
        .map( |(_,v)| v)
        .product();

    //generate values for CRT:
    //(a,v,m) where x = a mod m, v is an index, x is solution
    let crt_vals: Vec<(usize,usize,usize)>=
        enumed_table.iter()
        .map( |&(i,v)| (v-(i % v),v,big_modulo/v))
        .collect();
    
    let mut sum = 0;

    for (i,b,m) in crt_vals{
        for n in 0..{
            if ((m * n) % b) == 1{
                sum += i * m * n;
                break
            }
        }
    }
    
    p2 = sum % big_modulo;
    answer(p1,p2)

}