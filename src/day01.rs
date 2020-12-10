use crate::utils::answer;
use std::collections::HashSet;
use itertools::Itertools;

pub fn day01(input: String) -> (String,String){
    let mut p1 = 0;
    let mut p2 = 0;
    // part 1: put all the numbers in a set, iteratively check if a number's complement is in the set.
    // just woke up this is where my brain's at. O(nlogn).
    let mut numset: HashSet<i32> = HashSet::new();

    let nums = input.lines()
        .map(|n| n.parse::<i32>().expect("Failed to read line!"));

    for n in nums.clone(){
        numset.insert(n);
    }

    for n in nums.clone(){
        let m = 2020 - n;
        if numset.contains(&m) {
            p1 = n * m;
            break;
        }
    }

    //part 2: loop over cartesian product of nums, seeing if the sum of the pair's complement is in the set.
    //O(n^2), good enough.
    for (a,b) in nums.clone().cartesian_product(nums){
        let c = 2020 - (a+b);
        if numset.contains(&c){
            p2 = a * b * c;
            break;
        }
    }

    answer(p1,p2)
}
