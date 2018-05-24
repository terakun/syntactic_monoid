use regex;
use nfa;
use nfa::NFA;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct State {
    t: Vec<i32>,
    id: i32,
    pub accept: bool,
}

impl State {
    pub fn new(id: i32, accept: bool) -> Self {
        State {
            t: vec![-1; 256],
            id: id,
            accept: accept,
        }
    }
    pub fn set_trans(&mut self, transition: &Vec<i32>) {
        self.t = transition.clone();
    }
    pub fn transition(&self, c: u8) -> i32 {
        self.t[c as usize]
    }
    pub fn add_trans(&mut self, id: i32, ch: usize) {
        self.t[ch] = id;
    }
    pub fn print_trans(&self) {
        let mut i = 0;
        for t in &self.t {
            if *t != -1 {
                print!("{} by {} ", *t, i as u8 as char);
            }
            i = i + 1;
        }
        println!("");
    }
}

#[derive(Debug, Clone)]
pub struct DFA {
    pub states: Vec<State>,
    start: usize,
    is_minimum: bool,
}

impl DFA {
    pub fn new() -> Self {
        DFA {
            states: Vec::new(),
            start: 0,
            is_minimum: false,
        }
    }
    pub fn size(&self) -> usize {
        self.states.len()
    }

    pub fn get_trans(&self, i: usize, c: u8) -> i32 {
        self.states[i].transition(c)
    }

    pub fn add_state(&mut self, s: State) {
        self.states.push(s);
    }
    fn construct(re: &regex::RegularExpression) -> DFA {
        DFA::new()
    }

    fn construct_from_nfa(nfa: &NFA) -> Self {
        DFA::new()
    }

    pub fn minimize(&self) -> Self {
        let mut min_dfa = self.reduction();
        let mut size = self.size();
        loop {
            if min_dfa.size() == size {
                break;
            }
            size = min_dfa.size();
            min_dfa = min_dfa.reduction();
        }
        min_dfa
    }

    pub fn reduction(&self) -> Self {
        let mut transition_map: HashMap<(Vec<i32>, bool), Vec<i32>> = HashMap::new();
        for s in &self.states {
            transition_map.insert((s.t.clone(), s.accept), Vec::new());
        }

        for s in &self.states {
            if let Some(mut v) = transition_map.get_mut(&(s.t.clone(), s.accept)) {
                v.push(s.id);
            }
        }

        let mut id_vec: Vec<i32> = vec![-1; 256];
        for (min_id, m) in transition_map.iter().enumerate() {
            for id in m.1 {
                id_vec[*id as usize] = min_id as i32;
            }
        }

        let mut min_dfa = DFA {
            states: Vec::new(),
            start: id_vec[self.start] as usize,
            is_minimum: true,
        };

        for (min_id, m) in transition_map.iter().enumerate() {
            let trans = (m.0)
                .0
                .iter()
                .map(|t| if *t != -1 { id_vec[*t as usize] } else { -1 })
                .collect();
            min_dfa.add_state(State {
                t: trans,
                id: min_id as i32,
                accept: (m.0).1,
            });
        }
        min_dfa
    }

    pub fn to_graphviz(&self) {
        println!("digraph DFA {{");
        println!("  rankdir=\"LR\"");
        for s in &self.states {
            print!(" {} [ shape=", s.id);
            if s.accept {
                print!("doublecircle");
            } else {
                print!("circle");
            }
            println!(" ];");
        }
        println!(" start [ shape=plaintext ];");
        for s in &self.states {
            for (ch, t) in s.t.iter().enumerate() {
                if *t != -1 {
                    println!(" {} -> {} [ label = \"{}\" ];", s.id, t, ch as u8 as char);
                }
            }
            if s.id == self.start as i32 {
                println!(" start -> {}", s.id);
            }
        }
        println!("}}");
    }

    pub fn accept(&self, input: &Vec<u8>) -> bool {
        let mut state = self.start as i32;
        for c in input {
            state = self.states[state as usize].transition(*c);
            if state == -1 {
                return false;
            }
        }
        self.states[state as usize].accept
    }
}
