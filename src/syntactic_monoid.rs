use super::dfa::DFA;
use regex::RegularExpression;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::collections::BTreeSet;

extern crate bit_set;
use self::bit_set::BitSet;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Matrix {
    pub mat: Vec<Vec<u8>>,
    n: usize,
}

impl Matrix {
    fn new(size: usize) -> Self {
        Matrix {
            mat: vec![vec![0; size]; size],
            n: size,
        }
    }
    fn ident(size: usize) -> Self {
        let mut mat = vec![vec![0; size]; size];
        for i in 0..size {
            mat[i][i] = 1;
        }
        Matrix { mat: mat, n: size }
    }
    fn get(&self, i: usize, j: usize) -> u8 {
        self.mat[i][j]
    }
    fn set(&mut self, i: usize, j: usize, a: u8) {
        self.mat[i][j] = a;
    }
    fn to_string(&self) -> String {
        let mut s = String::new();
        for rvec in &self.mat {
            for c in rvec {
                s = format!("{} {}", s, c);
            }
            s = format!("{}\n", s);
        }
        s
    }
    fn multiply(&self, m: &Matrix) -> Self {
        let mut mat = vec![vec![0; self.n]; self.n];
        for i in 0..self.n {
            for j in 0..self.n {
                for k in 0..self.n {
                    mat[i][j] = mat[i][j] + self.mat[i][k] * m.mat[k][j];
                }
            }
        }
        Matrix {
            mat: mat,
            n: self.n,
        }
    }
}

type ElemType = usize;
type ElemSet = BTreeSet<ElemType>;

pub fn identity(e: &ElemType) -> bool {
    *e == 0
}

pub struct SyntacticMonoid {
    multiplication_table: Vec<Vec<ElemType>>,
    transitions_map: HashMap<Matrix, usize>,
    transitions: Vec<Matrix>,
    dfa: DFA,
    alphabets: BitSet,
    accept: Vec<bool>,
    charmorphism: HashMap<u8, ElemType>,
    deg: usize, // a number of elements
    input: String,
}

impl SyntacticMonoid {
    pub fn new() -> Self {
        SyntacticMonoid {
            multiplication_table: Vec::new(),
            transitions_map: HashMap::new(),
            transitions: Vec::new(),
            dfa: DFA::new(),
            alphabets: BitSet::with_capacity(256),
            accept: Vec::new(),
            charmorphism: HashMap::new(),
            deg: 0,
            input: String::new(),
        }
    }
    pub fn morphism(&self, text: String) -> ElemType {
        let mut mat = Matrix::new(self.dfa.size());
        let u8vec: Vec<u8> = text.chars().map(|x| x as u8).collect();
        for i in 0..self.dfa.size() {
            let mut state = i as i32;
            for c in &u8vec {
                if state != -1 {
                    state = self.dfa.get_trans(state as usize, *c);
                }
            }
            if state != -1 {
                mat.set(i, state as usize, 1);
            }
        }
        *self.transitions_map.get(&mat).unwrap()
    }

    pub fn make_elemset(&self) -> ElemSet {
        let mut m = ElemSet::new();
        for i in 0..self.deg {
            m.insert(i);
        }
        m
    }

    pub fn right_multiply(&self, s: &ElemSet, elem: ElemType) -> ElemSet {
        let mut m = ElemSet::new();
        for e in s.iter() {
            m.insert(self.multiplication_table[*e][elem]);
        }
        m
    }
    pub fn left_multiply(&self, elem: ElemType, s: &ElemSet) -> ElemSet {
        let mut m = ElemSet::new();
        for e in s.iter() {
            m.insert(self.multiplication_table[elem][*e]);
        }
        m
    }
    pub fn elemset_multiply(&self, s1: &ElemSet, s2: &ElemSet) -> ElemSet {
        let mut m = ElemSet::new();
        for e1 in s1.iter() {
            for e2 in s2.iter() {
                m.insert(self.multiplication_table[*e1][*e2]);
            }
        }
        m
    }

    pub fn aperiodic(&self) -> bool {
        for i in 0..self.deg {
            let mut e = i;
            for _ in 0..self.deg {
                e = self.multiplication_table[e][i];
            }
            if e != self.multiplication_table[e][i] {
                return false;
            }
        }
        true
    }

    pub fn accept(&self, e: &ElemType) -> bool {
        self.accept[*e]
    }

    pub fn construct(&mut self, dfa: &DFA, input: &String) {
        self.dfa = dfa.clone();
        self.input = input.clone();
        let ident = Matrix::ident(dfa.size());
        self.transitions_map.insert(ident.clone(), 0);
        let mut queue = VecDeque::new();
        queue.push_back(ident.clone());
        println!("dfa size:{}", dfa.size());
        while !queue.is_empty() {
            let mat = queue.front().unwrap().clone();
            for c in 0..256 {
                let mut next = Matrix::new(dfa.size());
                for i in 0..dfa.size() {
                    for j in 0..dfa.size() {
                        if mat.get(i, j) == 1 && dfa.get_trans(j, c as u8) != -1 {
                            next.set(i, dfa.get_trans(j, c as u8) as usize, 1);
                        }
                    }
                }
                if !self.transitions_map.contains_key(&next) {
                    let idx = self.transitions_map.len();
                    self.transitions_map.insert(next.clone(), idx);
                    queue.push_back(next.clone());
                }
                if mat == ident {
                    self.charmorphism
                        .insert(c as u8, *self.transitions_map.get(&next).unwrap());
                }
            }
            queue.pop_front().unwrap();
        }

        for p in &self.transitions_map {
            println!("mat({}) = \n{}", p.1, p.0.to_string());
        }

        self.deg = self.transitions_map.len();
        self.accept.resize(self.deg, false);
        self.transitions.resize(self.deg, Matrix::new(dfa.size()));
        self.multiplication_table
            .resize(self.deg, vec![0; self.deg]);
        for mat_i in &self.transitions_map {
            self.transitions[*mat_i.1] = mat_i.0.clone();
            self.accept[*mat_i.1] = false;

            for i in 0..dfa.size() {
                if mat_i.0.get(dfa.start, i) != 0 && dfa.states[i].accept {
                    self.accept[*mat_i.1] = true;
                }
            }
            for mat_j in &self.transitions_map {
                let mult_matij = mat_i.0.multiply(mat_j.0);
                self.multiplication_table[*mat_i.1][*mat_j.1] =
                    *self.transitions_map.get(&mult_matij).unwrap();
                let mult_matji = mat_j.0.multiply(mat_i.0);
                self.multiplication_table[*mat_j.1][*mat_i.1] =
                    *self.transitions_map.get(&mult_matji).unwrap();
            }
        }
    }
    pub fn starfree_expression(&self) -> Option<String> {
        if !self.aperiodic() {
            return None;
        }
        let mut memo: HashMap<ElemType, String> = HashMap::new();
        let mut regex_vec = vec!["@".to_string()];
        for e in 0..self.deg {
            if self.accept(&e) {
                regex_vec.push(self.starfree_recursion(e, &mut memo));
            }
        }
        let regex = regex_vec.join("|");
        Some(regex)
    }
    #[allow(non_snake_case)]
    fn starfree_recursion(&self, m: ElemType, memo: &mut HashMap<ElemType, String>) -> String {
        if let Some(ref s) = memo.get(&m) {
            return s.to_string();
        }
        let regex = if identity(&m) {
            let mut W = BitSet::with_capacity(256);
            for c in 0..256u32 {
                if !identity(&self.morphism((c as u8 as char).to_string())) {
                    W.insert(c as usize);
                }
            }
            if W.len() == 256 {
                "".to_string()
            } else if W.len() == 255 {
                let mut s = String::new();
                for c in 0..256 {
                    if !W.contains(c) {
                        s = format!("{}*", c as u8 as char);
                        break;
                    }
                }
                s
            } else {
                let mut s = String::new();
                for c in 0..256 {
                    if !W.contains(c) {
                        s = format!("{}{}", s, c as u8 as char);
                    }
                }
                format!("[{}]*", s)
            }
        } else {
            let M = self.make_elemset();
            let Mm = self.right_multiply(&M, m);
            let mM = self.left_multiply(m, &M);

            // build U A*
            let mut tmp: Vec<String> = Vec::new();
            for n in 0..(self.deg) {
                for a in 0..256u32 {
                    if mM.contains(&n) {
                        continue;
                    }
                    let na =
                        self.multiplication_table[n][self.morphism((a as u8 as char).to_string())];
                    let naM = self.left_multiply(na, &M);
                    if naM != mM {
                        continue;
                    }
                    tmp.push(self.starfree_recursion(n, memo) + &(a as u8 as char).to_string());
                }
            }
            let UA = if tmp.is_empty() {
                "@".to_string()
            } else if tmp.len() == 1 {
                format!("{}!@", tmp[0])
            } else {
                format!("({})!@", tmp.join("|"))
            };

            // build A* V
            let mut tmp: Vec<String> = Vec::new();
            for n in 0..(self.deg) {
                for a in 0..256u32 {
                    if Mm.contains(&n) {
                        continue;
                    }
                    let a_e = self.morphism((a as u8 as char).to_string());
                    let an = self.multiplication_table[a_e][n];
                    let Man = self.right_multiply(&M, an);
                    if Man != Mm {
                        continue;
                    }
                    tmp.push((a as u8 as char).to_string() + &self.starfree_recursion(n, memo));
                }
            }
            let AV = if tmp.is_empty() {
                "@".to_string()
            } else if tmp.len() == 1 {
                format!("!@{}", tmp[0])
            } else {
                format!("!@({})", tmp.join("|"))
            };

            // build A* W A*
            let mut W_: HashSet<u8> = HashSet::new();
            for a in 0..256u32 {
                let a_e = self.morphism((a as u8 as char).to_string());
                let aM = self.left_multiply(a_e, &M);
                let MaM = self.elemset_multiply(&M, &aM);
                if !MaM.contains(&m) {
                    W_.insert(a as u8);
                }
            }

            let mut tmp: Vec<String> = Vec::new();
            for a in 0..256u32 {
                let a_e = self.morphism((a as u8 as char).to_string());
                let Ma = self.right_multiply(&M, a_e);
                for b in 0..256u32 {
                    let b_e = self.morphism((b as u8 as char).to_string());
                    let bM = self.left_multiply(b_e, &M);
                    for n in 0..self.deg {
                        let nbM = self.left_multiply(n, &bM);
                        let ManbM = self.elemset_multiply(&Ma, &nbM);
                        if ManbM.contains(&m) {
                            continue;
                        }
                        let nM = self.left_multiply(n, &M);
                        let ManM = self.elemset_multiply(&Ma, &nM);
                        let MnbM = self.elemset_multiply(&M, &nbM);
                        if ManM.contains(&m) && MnbM.contains(&m) {
                            tmp.push(
                                (a as u8 as char).to_string() + &self.starfree_recursion(n, memo)
                                    + &(b as u8 as char).to_string(),
                            );
                        }
                    }
                }
            }

            let AWA = if W_.is_empty() && tmp.is_empty() {
                "@".to_string()
            } else {
                let mut s = String::new();
                if !W_.is_empty() {
                    if W_.len() == 1 {
                        for w in W_.iter() {
                            s = s + &(*w as u8 as char).to_string();
                        }
                    } else {
                        let mut f = BitSet::with_capacity(256);
                        for i in 0..256 {
                            f.insert(i);
                        }
                        let mut W_bit = BitSet::with_capacity(256);
                        for w in W_.iter() {
                            W_bit.insert(*w as usize);
                        }
                        for w in W_bit.symmetric_difference(&f) {
                            s = s + &(w as u8 as char).to_string();
                        }
                        s = format!("![{}]", s);
                    }
                    if !tmp.is_empty() {
                        s = s + &"|".to_string();
                    }
                }
                format!("!@({}{})!@", s, tmp.join("|"))
            };

            format!("!(!({})|!({})|{})", UA, AV, AWA)
        };
        memo.insert(m, regex.to_string());
        regex
    }
}
