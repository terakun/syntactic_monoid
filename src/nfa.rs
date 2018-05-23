use regex;
use regex::RegularExpression;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct State {
    ts: Vec<HashSet<usize>>,
    id: usize,
    pub accept: bool,
}

impl State {
    pub fn new(id: usize, accept: bool) -> Self {
        State {
            ts: vec![HashSet::new(); 256],
            id: id,
            accept: accept,
        }
    }
    pub fn add_trans(&mut self, id: usize, ch: usize) {
        self.ts[ch].insert(id);
    }
}

#[derive(Debug, Clone)]
pub struct NFA {
    pub states: Vec<State>,
    start: State,
    end: State,
}

impl NFA {
    pub fn new() -> Self {
        NFA {
            states: Vec::new(),
            start: State::new(0, false),
            end: State::new(0, false),
        }
    }
    pub fn size(&self) -> usize {
        self.states.len()
    }

    pub fn add_state(&mut self, s: State) {
        self.states.push(s);
    }

    pub fn shift_idx(&self, shift: usize) -> Self {
        let mut nfa = NFA::new();
        for s in &self.states {
            let mut new_ts: Vec<HashSet<usize>> = Vec::new();
            for t in &s.ts {
                let mut new_t: HashSet<usize> = HashSet::new();
                for q in t.iter() {
                    new_t.insert(q + shift);
                }
                new_ts.push(new_t);
            }
            let mut new_s = s.clone();
            new_s.id = s.id + shift;
            new_s.ts = new_ts;

            if new_s.id == self.start.id + shift {
                nfa.start = new_s.clone();
            } else if new_s.id == self.end.id + shift {
                nfa.end = new_s.clone();
            }

            nfa.add_state(new_s);
        }
        nfa
    }

    pub fn construct(re: &regex::RegularExpression) -> Self {
        match *re {
            RegularExpression::Empty => NFA::new(),
            RegularExpression::Epsilon => {
                let s = State::new(1, true);
                let mut nfa = NFA::new();
                nfa.add_state(s.clone());
                nfa.start = s.clone();
                nfa.end = s.clone();
                nfa
            }
            RegularExpression::Char(a) => {
                let mut s_i = State::new(0, false);
                let s_f = State::new(1, true);
                s_i.add_trans(1, a as usize);
                let mut nfa = NFA::new();
                nfa.add_state(s_i.clone());
                nfa.add_state(s_f.clone());
                nfa.start = s_i.clone();
                nfa.end = s_f.clone();
                nfa
            }
            RegularExpression::Concat(ref e1, ref e2) => {
                let mut nfa1 = NFA::construct(e1);
                let snum1 = nfa1.size();
                let mut nfa2 = NFA::construct(e2);

                let mut s_merged = nfa1.end.clone();
                let mut s_i2 = nfa2.start.clone();
                for (ch, t) in s_i2.ts.iter().enumerate() {
                    for q in t.iter() {
                        s_merged.add_trans(*q + snum1 - 1, ch);
                    }
                }
                s_merged.accept = false;

                let nfa2 = nfa2.shift_idx(snum1 - 1);
                let mut nfa = NFA::new();

                for s in &nfa1.states {
                    if s.id != nfa1.end.id {
                        nfa.add_state(s.clone());
                    }
                }
                nfa.add_state(s_merged);
                for s in &nfa2.states {
                    if s.id != nfa2.start.id {
                        nfa.add_state(s.clone());
                    }
                }
                nfa.start = nfa1.start;
                nfa.end = nfa2.end;
                nfa
            }
            RegularExpression::Union(ref e1, ref e2) => {
                let mut nfa1 = NFA::construct(e1);
                let snum1 = nfa1.size();
                let mut nfa2 = NFA::construct(e2);

                let mut s_merged_i = nfa1.start.clone();
                let s_i2 = nfa2.start.clone();
                for (ch, t) in s_i2.ts.iter().enumerate() {
                    for q in t.iter() {
                        if *q == nfa2.start.id {
                            s_merged_i.add_trans(nfa1.start.id, ch);
                        } else if *q == nfa2.end.id {
                            s_merged_i.add_trans(nfa1.end.id, ch);
                        } else {
                            s_merged_i.add_trans(*q + snum1 - 1, ch);
                        }
                    }
                }

                let mut s_merged_f = nfa1.end.clone();
                let mut s_f2 = nfa2.end.clone();
                for (ch, t) in s_f2.ts.iter().enumerate() {
                    for q in t.iter() {
                        if *q == nfa2.start.id {
                            s_merged_i.add_trans(nfa1.start.id, ch);
                        } else if *q == nfa2.end.id {
                            s_merged_i.add_trans(nfa1.end.id, ch);
                        } else {
                            s_merged_i.add_trans(*q + snum1 - 1, ch);
                        }
                    }
                }

                let mut nfa = NFA::new();
                for s in nfa1.states {
                    if s.id == nfa1.start.id {
                        nfa.add_state(s_merged_i.clone());
                    } else if s.id == nfa1.end.id {
                        nfa.add_state(s_merged_f.clone());
                    } else {
                        nfa.add_state(s);
                    }
                }

                let nfa2 = nfa2.shift_idx(snum1 - 1);
                for s in nfa2.states {
                    if s.id == nfa2.start.id || s.id == nfa2.end.id {
                        continue;
                    }
                    let mut new_ts: Vec<HashSet<usize>> = Vec::new();
                    for t in &s.ts {
                        let mut new_t: HashSet<usize> = HashSet::new();
                        for q in t.iter() {
                            if *q == nfa2.start.id {
                                new_t.insert(nfa1.start.id);
                            } else if *q == nfa2.end.id {
                                new_t.insert(nfa1.end.id);
                            } else {
                                new_t.insert(*q);
                            }
                        }
                        new_ts.push(new_t);
                    }
                    let mut new_s = State::new(s.id, false);
                    new_s.ts = new_ts;
                    nfa.add_state(new_s);
                }
                nfa.start = s_merged_i;
                nfa.end = s_merged_f;
                nfa
            }
            RegularExpression::Kleene(ref e) => {
                let mut nfa = NFA::construct(e);
                let s_f = nfa.end.clone();
                let mut s_merged = State::new(nfa.start.id, true);
                for (ch, t) in nfa.start.ts.iter().enumerate() {
                    if t.is_empty() {
                        continue;
                    }
                    print!("{}:", ch as u8 as char);
                    for q in t.iter() {
                        print!("{} ", *q);
                        if *q == nfa.end.id {
                            s_merged.add_trans(nfa.start.id, ch);
                        } else {
                            s_merged.add_trans(*q, ch);
                        }
                    }
                    println!("");
                }
                for (ch, t) in s_f.ts.iter().enumerate() {
                    for q in t.iter() {
                        if *q == nfa.end.id {
                            s_merged.add_trans(nfa.start.id, ch);
                        } else {
                            s_merged.add_trans(*q, ch);
                        }
                    }
                }
                let mut new_nfa = NFA::new();
                new_nfa.add_state(s_merged.clone());

                for s in &nfa.states {
                    if s.id == nfa.start.id || s.id == nfa.end.id {
                        continue;
                    }
                    let mut new_ts: Vec<HashSet<usize>> = Vec::new();
                    for t in &s.ts {
                        let mut new_t: HashSet<usize> = HashSet::new();
                        for q in t.iter() {
                            if *q == nfa.end.id {
                                new_t.insert(nfa.start.id);
                            } else {
                                new_t.insert(*q);
                            }
                        }
                        new_ts.push(new_t);
                    }
                    let mut new_s = State::new(s.id, false);
                    new_s.ts = new_ts;
                    new_nfa.add_state(new_s);
                }
                new_nfa.start = s_merged.clone();
                new_nfa.end = s_merged;
                new_nfa
            }
            _ => {
                panic!("not implemented");
            }
        }
    }

    pub fn to_graphviz(&self) {
        println!("digraph NFA {{");
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
            for (ch, t) in s.ts.iter().enumerate() {
                for q in t.iter() {
                    println!(" {} -> {} [ label = \"{}\" ];", s.id, q, ch as u8 as char);
                }
            }
            if s.id == self.start.id {
                println!(" start -> {}", s.id);
            }
        }
        println!("}}");
    }
}
