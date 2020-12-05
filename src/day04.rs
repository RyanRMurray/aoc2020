use regex::Regex;
use std::collections::HashMap;

fn valid_fields(pp:&HashMap<&str,&str>) -> bool{
    let kcount = pp.keys().count();
    kcount == 8 ||
    (kcount == 7 && !(pp.contains_key("cid")))
}

fn getf<'a>(pp: &'a HashMap<&str,&str>, f:&str) -> &'a str{
    match pp.get(f){
        Some(v) => v,
        _       => panic!("Invalid field!")
    }
}

//behold! the ugliest thing i have ever written!
fn validate_fields(pp: &HashMap<&str,&str>) -> bool{
    let eyes = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];  
    let hgt = getf(pp, "hgt");
    let hval: i16;
    
    if hgt.chars().last() == Some('m'){
        match hgt.get(0..3){
            None    => panic!("Invalid height!"),
            Some(s) => {
                match s.parse::<i16>(){
                    Ok(v) => hval = v,
                    _       => return false
                }
            }
        };
        if hval < 150 || hval > 193{
            return false
        }
    }else if hgt.chars().last() == Some('n'){
        match hgt.get(0..2){
            None    => panic!("Invalid height!"),
            Some(s) => {
                match s.parse::<i16>(){
                    Ok(v) => hval = v,
                    _       => return false
                }
            }
        };
        if hval < 59 || hval > 76{
            return false
        }
    }else{
        return false
    }

    getf(pp, "byr").parse::<i16>().expect(":(") > 1919 && 
    getf(pp, "byr").parse::<i16>().expect(":(") < 2003 &&
    getf(pp, "iyr").parse::<i16>().expect(":(") > 2009 && 
    getf(pp, "iyr").parse::<i16>().expect(":(") < 2021 &&
    getf(pp, "eyr").parse::<i16>().expect(":(") > 2019 && 
    getf(pp, "eyr").parse::<i16>().expect(":(") < 2031 &&
    getf(pp, "pid").len() == 9                         &&
    getf(pp, "hcl").chars().next() == Some('#')        &&
    getf(pp, "hcl").len() == 7                         &&
    eyes.contains(&getf(pp, "ecl"))
}

pub fn day04(input:String){
    //parse passports
    let mut passports: Vec<HashMap<&str,&str>> = Vec::new();
    let re =
        Regex::new(
            r"(?P<t>\S+):(?P<v>\S+)"
        ).unwrap();

    for pp in input.split("\n\n"){
        let mut pass: HashMap<&str,&str> = HashMap::new();
        for m in re.captures_iter(pp){
            pass.insert(
                m.name("t").unwrap().as_str(),
                m.name("v").unwrap().as_str()
            );
        }
        passports.push(pass.clone())
    }

    //part 1: find number of valid passports
    let valids = passports.iter().filter(|pp| valid_fields(pp));
    println!("Part 1: {}", valids.clone().count());

    //part 2: validate fields
    let valids2 = valids.filter(|pp| validate_fields(pp));
    println!("Part 1: {}", valids2.count());
}