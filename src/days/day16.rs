extern crate regex;

use self::regex::Regex;

struct Aunt {
    children: Option<usize>,
    cats: Option<usize>,
    samoyeds: Option<usize>,
    pomeranians: Option<usize>,
    akitas: Option<usize>,
    vizslas: Option<usize>,
    goldfish: Option<usize>,
    trees: Option<usize>,
    cars: Option<usize>,
    perfumes: Option<usize>,
}

impl Aunt {

    fn cmp(&self, aunt: &Aunt) -> bool {
        // most beautiful code
        if let Some(n) = self.children {
            if let Some(m) = aunt.children {
                if n != m {return false;}
            }
        }
        if let Some(n) = self.cats {
            if let Some(m) = aunt.cats {
                if n >= m {return false;}
            }
        }
        if let Some(n) = self.samoyeds {
            if let Some(m) = aunt.samoyeds {
                if n != m {return false;}
            }
        }
        if let Some(n) = self.pomeranians {
            if let Some(m) = aunt.pomeranians {
                if n <= m {return false;}
            }
        }
        if let Some(n) = self.akitas {
            if let Some(m) = aunt.akitas {
                if n != m {return false;}
            }
        }
        if let Some(n) = self.vizslas {
            if let Some(m) = aunt.vizslas {
                if n != m {return false;}
            }
        }
        if let Some(n) = self.goldfish {
            if let Some(m) = aunt.goldfish {
                if n <= m {return false;}
            }
        }
        if let Some(n) = self.trees {
            if let Some(m) = aunt.trees {
                if n >= m {return false;}
            }
        }
        if let Some(n) = self.cars {
            if let Some(m) = aunt.cars {
                if n != m {return false;}
            }
        }
        if let Some(n) = self.perfumes {
            if let Some(m) = aunt.perfumes {
                if n != m {return false;}
            }
        }
        true
    }

    fn from_str(s: &str) -> Aunt {
        let mut children = None;
        let mut cats = None;
        let mut samoyeds = None;
        let mut pomeranians = None;
        let mut akitas = None;
        let mut vizslas = None;
        let mut goldfish = None;
        let mut trees = None;
        let mut cars = None;
        let mut perfumes = None;

        let parse = Regex::new(r"(\w+): (\d+)").expect("illegal regex");

        for capture in parse.captures_iter(s) {
            let name = capture.at(1).unwrap();
            let val = capture.at(2).unwrap().parse::<usize>().unwrap();
            match name {
                "children" => children = Some(val),
                "cats" => cats = Some(val),
                "samoyeds" =>  samoyeds = Some(val),
                "pomeranians" => pomeranians = Some(val),
                "akitas" =>  akitas = Some(val),
                "vizslas" =>  vizslas = Some(val),
                "goldfish" =>  goldfish = Some(val),
                "trees" => trees = Some(val),
                "cars" =>  cars = Some(val),
                "perfumes" =>  perfumes = Some(val),
                _ => println!("warn: what is this? {}", name)
            }

        }
        Aunt {
            children: children,
            cats: cats,
            samoyeds: samoyeds,
            pomeranians: pomeranians,
            akitas: akitas,
            vizslas: vizslas,
            goldfish: goldfish,
            trees: trees,
            cars: cars,
            perfumes: perfumes
        }
    }
}


pub fn day_16(input: String) {
    let goal = Aunt {
        children: Some(3),
        cats: Some(7), // <
        samoyeds: Some(2),
        pomeranians: Some(3), // >
        akitas: Some(0),
        vizslas: Some(0),
        goldfish: Some(5), // >
        trees: Some(3), // <
        cars: Some(2),
        perfumes: Some(1)
    };

    let aunts: Vec<(Aunt, usize)> = input.lines().map(Aunt::from_str).zip(1..501).collect();
    let cands: Vec<_> = aunts.into_iter().filter(|&(ref a, _)| goal.cmp(&a)).collect();

    for (_, i) in cands {
        println!("{}", i);
    }
}
