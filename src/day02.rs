use crate::utils::answer;
use regex::Regex;

#[derive(Debug, Clone)]
struct Password {
    fst: usize,
    snd: usize,
    sym: char,
    pwd: String
}

impl Password {
    fn verify(&self) -> bool{
        let matches = self.pwd.matches(self.sym).count();
        
        self.fst <= matches && matches <= self.snd
    }

    fn match_char(&self, i: usize) -> bool{
        match self.pwd.chars().nth(i-1){
            Some(c) => c == self.sym,
            None    => false
        }
    }

    fn other_verify(&self) -> bool{
        let fst = self.match_char(self.fst);
        let snd = self.match_char(self.snd);

        !(fst && snd) && (fst || snd)
    }
}

pub fn day02(input: String) -> (String,String){
    let p1;
    let p2;
    let re =
        Regex::new(
            r"(\d+)-(\d+) (.): (\w+)"
        )
        .unwrap();
    
    let mut pwds: Vec::<Password> = Vec::new();

    //parse passwords into the above struct
    for l in input.lines(){
        for cap in re.captures_iter(l){
            pwds.push(Password{
                fst : cap[1].parse().expect(":("),
                snd : cap[2].parse().expect(":("),
                sym : cap[3].chars().next().unwrap(),
                pwd : cap[4].to_string()
            })
        }
    }

    //count valid
    p1 = 
        pwds.iter()
        .filter(|p| p.verify())
        .count();

    //count valid for part 2
    p2 = 
        pwds.iter()
        .filter(|p| p.other_verify())
        .count();

    answer(p1,p2)
}