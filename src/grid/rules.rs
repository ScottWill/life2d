use rand::random;


const RULES: [Rule; 10] = [
    Rule {
        name: "Conway's Life",
        flags: [
            [false, false, false,  true, false, false, false, false, false], // born
            [false, false,  true,  true, false, false, false, false, false], // survive
        ]
    },
    Rule {
        name: "3-4 Life",
        flags: [
            [false, false, false,  true,  true, false, false, false, false], // born
            [false, false, false,  true,  true, false, false, false, false], // survive
        ]
    },
    Rule {
        name: "Replicator",
        flags: [
            [false,  true, false,  true, false,  true, false,  true, false], // born
            [false,  true, false,  true, false,  true, false,  true, false], // survive
        ]
    },
    Rule {
        name: "Seeds",
        flags: [
            [false, false,  true, false, false, false, false, false, false], // born
            [false, false, false, false, false, false, false, false, false], // survive
        ]
    },
    Rule {
        name: "Long Life",
        flags: [
            [false, false, false,  true,  true,  true, false, false, false], // born
            [false, false, false, false, false,  true, false, false, false], // survive
        ]
    },
    Rule {
        name: "Parallels",
        flags: [
            [false, false, false,  true, false, false, false, false, false], // born
            [ true,  true,  true,  true, false, false, false, false, false], // survive
        ]
    },
    Rule {
        name: "A-maze-ing",
        flags: [
            [false, false, false,  true, false, false, false, false, false], // born
            [ true,  true,  true,  true,  true, false, false, false, false], // survive
        ]
    },
    Rule {
        name: "No Death",
        flags: [
            [false, false, false,  true, false, false, false, false, false], // born
            [ true,  true,  true,  true,  true,  true,  true,  true,  true], // survive
        ]
    },
    Rule {
        name: "Day & Night",
        flags: [
            [false, false, false,  true, false, false,  true,  true,  true], // born
            [false, false, false,  true,  true, false,  true,  true,  true], // survive
        ]
    },
    Rule {
        name: "Walled Cities",
        flags: [
            [false, false, false, false,  true,  true,  true,  true,  true], // born
            [false, false,  true,  true,  true,  true, false,  true, false], // survive
        ]
    }
];

#[derive(Clone)]
pub struct Rule<'a> {
    name: &'a str,
    flags: [[bool; 9]; 2]
}

#[derive(Default)]
pub struct Rules {
    rule: usize,
}

impl Rules {
    
    pub fn eval(&self, state: bool, count: usize) -> bool {
        RULES[self.rule].flags[state as usize][count]
    }

    pub fn name(&self) -> &str {
        RULES[self.rule].name
    }
    
    pub fn next_rule(&mut self) {
        self.set_rule(/*RULES.len() +*/ self.rule + 1);
    }
    
    pub fn prev_rule(&mut self) {
        self.set_rule(RULES.len() + self.rule - 1);
    }
    
    pub fn random_rule(&mut self) {
        self.set_rule(random::<usize>());
    }
    
    pub fn reset_rules(&mut self) {
        self.rule = 0;
    }

    fn set_rule(&mut self, value: usize) {
        self.rule = value % RULES.len();
    }
    
}