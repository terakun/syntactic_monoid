use nfa::NFA;
use nfa::SubSet;
use std::collections::HashMap;
use std::collections::BTreeSet;
use std::collections::BTreeMap;
use std::collections::VecDeque;

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
    pub start: usize,
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

    pub fn print_subset(subset: &SubSet) {
        for s in subset {
            print!("{} ", s);
        }
        println!("");
    }
    pub fn construct_from_nfa(nfa: &NFA) -> Self {
        let mut family: BTreeSet<SubSet> = BTreeSet::new();

        let s_i = nfa.start.clone();
        let mut subset_i = SubSet::new();
        subset_i.insert(s_i.id);
        let expand_subset_i = nfa.epsilon_expand(&subset_i);

        let mut queue: VecDeque<SubSet> = VecDeque::new();
        queue.push_back(expand_subset_i);
        while !queue.is_empty() {
            let subset = queue.pop_front().unwrap();
            let subset = nfa.epsilon_expand(&subset);
            if family.contains(&subset) {
                continue;
            }
            family.insert(subset.clone());
            for ch in 0..256 {
                let mut next = SubSet::new();
                for s in &subset {
                    for q in nfa.states[*s].ts[ch].iter() {
                        next.insert(*q);
                    }
                }
                if !next.is_empty() {
                    queue.push_back(next);
                }
            }
        }
        let mut nfa2dfa_id: HashMap<SubSet, usize> = HashMap::new();
        for (dfa_id, subset) in family.iter().enumerate() {
            nfa2dfa_id.insert(subset.clone(), dfa_id);
        }

        let mut dfa = DFA::new();

        for (dfa_id, subset) in family.iter().enumerate() {
            let mut dfa_state = State::new(dfa_id as i32, false);
            for ch in 0..256 {
                let mut trans_subset = SubSet::new();
                for s in subset {
                    for q in nfa.states[*s].ts[ch].iter() {
                        trans_subset.insert(*q);
                    }
                }
                if trans_subset.is_empty() {
                    continue;
                }
                let trans_subset = nfa.epsilon_expand(&trans_subset);
                dfa_state.t[ch as usize] = *nfa2dfa_id.get(&trans_subset).unwrap() as i32;
            }
            for s in subset {
                if nfa.states[*s].id == nfa.start.id {
                    dfa.start = nfa.start.id;
                }
                if nfa.states[*s].accept {
                    dfa_state.accept = true;
                }
            }
            dfa.add_state(dfa_state);
        }
        dfa
    }

    pub fn minimize(&self) -> Self {
        let mut min_dfa = self.reduction();
        loop {
            let next_dfa = min_dfa.reduction();
            if min_dfa.size() == next_dfa.size() {
                break;
            }
            min_dfa = next_dfa;
        }
        min_dfa
    }

    pub fn reduction(&self) -> Self {
        // 遷移関数が同じ && 受理状態が同じ 状態を同一視する
        let mut transition_map: BTreeMap<(Vec<i32>, bool), Vec<i32>> = BTreeMap::new();
        for s in &self.states {
            transition_map.insert((s.t.clone(), s.accept), Vec::new());
        }

        for s in &self.states {
            if let Some(mut v) = transition_map.get_mut(&(s.t.clone(), s.accept)) {
                v.push(s.id);
            }
        }

        let mut min_dfa_ids: Vec<i32> = vec![-1; self.size()];
        for (min_id, m) in transition_map.iter().enumerate() {
            for id in m.1 {
                min_dfa_ids[*id as usize] = min_id as i32;
            }
        }

        let mut min_dfa = DFA {
            states: Vec::new(),
            start: min_dfa_ids[self.start] as usize,
            is_minimum: true,
        };

        for (min_id, m) in transition_map.iter().enumerate() {
            let trans = (m.0)
                .0
                .iter()
                .map(|t| {
                    if *t != -1 {
                        min_dfa_ids[*t as usize]
                    } else {
                        -1
                    }
                })
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
            print!("  {} [ shape=", s.id);
            if s.accept {
                print!("doublecircle");
            } else {
                print!("circle");
            }
            println!(" ];");
        }
        println!("  start [ shape=plaintext ];");
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
