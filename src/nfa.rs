use regex;
use regex::RegularExpression;
use std::collections::HashSet;
use std::collections::BTreeSet;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct State {
    pub ts: Vec<HashSet<usize>>,
    pub epsilon: HashSet<usize>,
    pub id: usize,
    pub accept: bool,
}

pub type SubSet = BTreeSet<usize>;

impl State {
    pub fn new(id: usize, accept: bool) -> Self {
        State {
            ts: vec![HashSet::new(); 256],
            epsilon: HashSet::new(),
            id: id,
            accept: accept,
        }
    }

    pub fn add_epsilon(&mut self, id: usize) {
        self.epsilon.insert(id);
    }
    pub fn add_trans(&mut self, id: usize, ch: usize) {
        self.ts[ch].insert(id);
    }
    pub fn print_trans(&self) {
        for (ch, t) in self.ts.iter().enumerate() {
            if t.is_empty() {
                continue;
            }
            print!("{}:", ch as u8 as char);
            for q in t.iter() {
                print!("{} ", *q);
            }
            println!("");
        }
    }
}

#[derive(Debug, Clone)]
pub struct NFA {
    pub states: Vec<State>,
    pub start: State,
    pub end: State,
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

    pub fn epsilon_expand(&self, subset: &SubSet) -> SubSet {
        let mut expand_subset = SubSet::new();
        let mut queue: VecDeque<usize> = VecDeque::new();
        for q in subset.iter() {
            queue.push_back(*q);
            expand_subset.insert(*q);
        }
        while !queue.is_empty() {
            let state = queue.pop_front().unwrap();
            for q in self.states[state].epsilon.iter() {
                if !expand_subset.contains(q) {
                    queue.push_back(*q);
                    expand_subset.insert(*q);
                }
            }
        }
        expand_subset
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
            let mut new_epsilon: HashSet<usize> = HashSet::new();
            for q in s.epsilon.iter() {
                new_epsilon.insert(*q + shift);
            }
            let mut new_s = s.clone();
            new_s.id = s.id + shift;
            new_s.ts = new_ts;
            new_s.epsilon = new_epsilon;

            if new_s.id == self.start.id + shift {
                nfa.start = new_s.clone();
            }
            if new_s.id == self.end.id + shift {
                nfa.end = new_s.clone();
            }

            nfa.add_state(new_s);
        }
        nfa
    }

    // pub fn fix_id(&self) -> Self {
    //     let mut ids: HashMap<usize, usize> = HashMap::new();
    //     let mut fixed_nfa = NFA::new();
    //     for (id, s) in self.states.iter().enumerate() {
    //         ids.insert(s.id, id);
    //     }
    //     for s in &self.states {
    //         let mut new_ts: Vec<HashSet<usize>> = Vec::new();
    //         for t in &s.ts {
    //             let mut new_t: HashSet<usize> = HashSet::new();
    //             for q in t.iter() {
    //                 new_t.insert(*ids.get(q).unwrap());
    //             }
    //             new_ts.push(new_t);
    //         }
    //         let mut new_s = s.clone();
    //         new_s.ts = new_ts;
    //         new_s.id = *ids.get(&s.id).unwrap();
    //         if s.id == self.start.id {
    //             fixed_nfa.start = new_s.clone();
    //         }
    //         if s.id == self.end.id {
    //             fixed_nfa.end = new_s.clone();
    //         }
    //         fixed_nfa.add_state(new_s);
    //     }
    //     fixed_nfa
    // }
    pub fn construct(re: &regex::RegularExpression) -> Self {
        match *re {
            RegularExpression::Empty => NFA::new(),
            RegularExpression::Epsilon => {
                let mut s_i = State::new(0, false);
                let mut s_f = State::new(1, true);
                s_i.add_epsilon(1);
                let mut nfa = NFA::new();
                nfa.add_state(s_i.clone());
                nfa.add_state(s_f.clone());
                nfa.start = s_i;
                nfa.end = s_f;
                nfa
            }
            RegularExpression::Char(a) => {
                let mut s_i = State::new(0, false);
                let s_f = State::new(1, true);
                s_i.add_trans(1, a as usize);
                let mut nfa = NFA::new();
                nfa.add_state(s_i.clone());
                nfa.add_state(s_f.clone());
                nfa.start = s_i;
                nfa.end = s_f;
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
                for q in s_i2.epsilon.iter() {
                    s_merged.add_epsilon(*q + snum1 - 1);
                }

                let nfa2 = nfa2.shift_idx(snum1 - 1);
                let mut nfa = NFA::new();
                for s in &nfa1.states {
                    if s.id != nfa1.end.id {
                        nfa.add_state(s.clone());
                    }
                }
                nfa.add_state(s_merged.clone());
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
                let nfa1 = NFA::construct(e1);
                let snum1 = nfa1.size();
                let nfa2 = NFA::construct(e2);
                let snum2 = nfa2.size();

                let mut s_i = State::new(0, false);
                s_i.add_epsilon(nfa1.start.id + 1);
                s_i.add_epsilon(nfa1.start.id + snum1 + 1);

                let s_f = State::new(snum1 + snum2 + 1, true);
                let mut nfa = NFA::new();
                nfa.add_state(s_i.clone());
                let nfa1 = nfa1.shift_idx(1);
                let mut s_f1 = nfa1.end.clone();
                s_f1.add_epsilon(s_f.id);
                s_f1.accept = false;
                for s in nfa1.states {
                    if s.id == nfa1.end.id {
                        nfa.add_state(s_f1.clone());
                    } else {
                        nfa.add_state(s);
                    }
                }

                let nfa2 = nfa2.shift_idx(snum1 + 1);
                let mut s_f2 = nfa2.end.clone();
                s_f2.add_epsilon(s_f.id);
                s_f2.accept = false;
                for s in nfa2.states {
                    if s.id == nfa2.end.id {
                        nfa.add_state(s_f2.clone());
                    } else {
                        nfa.add_state(s);
                    }
                }
                nfa.add_state(s_f.clone());
                nfa.start = s_i;
                nfa.end = s_f;
                nfa
            }
            RegularExpression::Kleene(ref e) => {
                let mut nfa = NFA::construct(e);
                let nfa = nfa.shift_idx(1);
                let snum = nfa.size();
                let mut s_i = State::new(0, false);
                s_i.add_epsilon(nfa.start.id);
                let mut s_f = State::new(snum + 1, true);
                let mut s_e_i = nfa.start.clone();
                s_e_i.add_epsilon(s_f.id);
                let mut s_e_f = nfa.end.clone();
                s_e_f.accept = false;
                s_e_f.add_epsilon(s_e_i.id);

                let mut new_nfa = NFA::new();
                new_nfa.add_state(s_i.clone());
                for s in nfa.states {
                    if s.id == nfa.start.id {
                        new_nfa.add_state(s_e_i.clone());
                    } else if s.id == nfa.end.id {
                        new_nfa.add_state(s_e_f.clone());
                    } else {
                        new_nfa.add_state(s);
                    }
                }
                new_nfa.add_state(s_f.clone());

                new_nfa.start = s_i;
                new_nfa.end = s_f;
                new_nfa
            }
        }
    }

    pub fn to_graphviz(&self) {
        println!("digraph NFA {{");
        println!("  rankdir=\"LR\"");
        for s in &self.states {
            print!("  {} [ shape=", s.id);
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
            for q in s.epsilon.iter() {
                println!(" {} -> {} [ label = ε ];", s.id, q);
            }

            if s.id == self.start.id {
                println!(" start -> {}", s.id);
            }
        }
        println!("}}");
    }
}
