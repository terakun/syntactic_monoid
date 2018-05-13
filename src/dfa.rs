use regex;

#[derive(Debug, Clone)]
pub struct State {
    t: Vec<i32>,
    id: i32,
    accept: bool,
}

impl State {
    pub fn new(id: i32, accept: bool) -> Self {
        State {
            t: vec![-1; 256],
            id: id,
            accept: accept,
        }
    }
    pub fn get(&self, i: usize) -> i32 {
        self.t[i]
    }
    pub fn transition(&self, c: u8) -> i32 {
        self.t[c as usize]
    }
    pub fn add_trans(&mut self, id: i32, ch: char) {
        let ch = ch as usize;
        self.t[ch] = id;
    }
}

#[derive(Debug, Clone)]
pub struct DFA {
    pub states: Vec<State>,
}

impl DFA {
    pub fn new() -> Self {
        DFA { states: Vec::new() }
    }
    fn size(&self) -> usize {
        self.states.len()
    }

    pub fn add_state(&mut self, s: State) {
        self.states.push(s);
    }
    fn construct(re: &regex::RegularExpression) -> DFA {
        DFA { states: Vec::new() }
    }
    fn minimize(&self) -> Self {
        self.clone()
    }

    pub fn to_graphviz(&self) {
        println!("digraph DFA {{");
        for s in &self.states {
            print!(" {} [ shape=", s.id);
            if s.accept {
                print!("doublecircle");
            } else {
                print!("circle");
            }
            println!(" ];");
        }
        for s in &self.states {
            for t in &s.t {
                if *t != -1 {
                    println!(" {} -> {};", s.id, t);
                }
            }
        }
        println!("}}");
    }

    fn accept(&self, input: &Vec<u8>) -> bool {
        let mut state = 0;
        for c in input {
            state = self.states[state as usize].transition(*c);
            if state == -1 {
                return false;
            }
        }
        self.states[state as usize].accept
    }
}
