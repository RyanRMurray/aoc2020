pub fn day10(input:String){
    //kind of a messy input but whatever
    let mut adaptors: Vec<u32>= vec![0];
    let mut ins: Vec<u32> = 
        input.lines()
        .map( |s|
            s.parse().expect(":(")
        )
        .collect();

    adaptors.append(&mut ins);
    adaptors.sort();
    adaptors.push(adaptors.last().unwrap() + 3);

    //Part 1: Find product of 1-distance and 3-distances
    let mut jolt_lv: u32 = 0;
    let mut count_1 = 0;
    let mut count_3 = 0;

    for a in adaptors.iter(){
        match a - jolt_lv{
            0 => (),
            1 => count_1 += 1,
            2 => (),
            3 => count_3 += 1,
            _ => break
        }

        jolt_lv = *a;
    }
    println!("Part 1: {}", count_1 * count_3);

    //Part 2: find all adaptors that reach an adaptor, adding the number
    //of ways those adaptors can be reached, assuming "0" can be reached
    //in exactly one way
    let mut paths_to: Vec<u64> = vec![0;adaptors.len()];
    paths_to[0] = 1;

    for to in 1..adaptors.len(){
        for from in 1..{
            if from < to + 1 && adaptors[to] - adaptors[to-from] < 4{
                paths_to[to] += paths_to[to-from]
            }else{
                break
            }
        }
    }

    println!("Part 2: {}", paths_to.last().unwrap());
}