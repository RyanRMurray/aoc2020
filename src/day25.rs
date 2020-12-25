use crate::utils::*;

pub fn day25(input:String) -> (String,String){
    let p1;
    let p2 = "Merry Christmas, ya filthy animal";

    //parse public keys
    let ins: Vec<u64> = input.lines().map(|v| v.parse().unwrap()).collect();

    //part 1: find encryption key
    let mut loop_nums = vec![];

    for public_key in ins.iter(){
        let mut val = 1;
        let mut loops = 0;

        while val != *public_key{
            val *= 7;
            val = val % 20201227;
            loops += 1;
        }
        loop_nums.push(loops);
    }

    p1 = 
        (0..loop_nums[0])
        .fold(1, |acc,_| {let v = acc * ins[1]; v % 20201227});

    

    answer(p1,p2)
}