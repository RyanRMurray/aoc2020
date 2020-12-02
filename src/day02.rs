#[derive(Debug, Clone)]
struct Password {
    range: (usize,usize),
    sym: char,
    pwd: String
}

impl Password {

    fn verify(&self) -> bool{
        let matches = self.pwd.matches(self.sym).count();
        
        self.range.0 <= matches && matches <= self.range.1
    }

    fn match_char(&self, i: usize) -> bool{
        match self.pwd.chars().nth(i-1){
            Some(c) => c == self.sym,
            None    => false
        }
    }

    fn other_verify(&self) -> bool{
        let fst = self.match_char(self.range.0);
        let snd = self.match_char(self.range.1);

        !(fst && snd) && (fst || snd)
    }
}

pub fn day02(input: String){
    //parse passwords into the above struct
    let pwds_it: Vec<Vec<&str>> = input.lines().map(|l| l.split(" ").collect()).collect();
    let mut pwds: Vec::<Password> = Vec::new();
    let mut valid = 0;

    for l in pwds_it{
        let rs: Vec<usize> = l[0].split("-").map(|n| n.parse().expect(":(")).collect();
        pwds.push(Password {
            range : (rs[0], rs[1]),
            sym   : l[1].chars().nth(0).unwrap(),
            pwd   : String::from(l[2])
        })
    }

    //count valid
    for p in pwds.clone(){
        if p.verify(){
            valid += 1;
        }
    }

    println!("Part 1: {}", valid);
    valid = 0;

    //count valid for part 2
    for p in pwds{
        if p.other_verify(){
            valid += 1;
        }
    }
    
    println!("Part 2: {}", valid);
}