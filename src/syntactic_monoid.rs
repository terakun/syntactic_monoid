use super::regex;
use super::dfa::DFA;
use regex::RegularExpression;
use std::collections::HashMap;
use std::collections::VecDeque;

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
    fn Ident(size: usize) -> Self {
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
    deg: usize, // a number of elements
}

impl SyntacticMonoid {
    pub fn new() -> Self {
        SyntacticMonoid {
            multiplication_table: Vec::new(),
            transitions_map: HashMap::new(),
            transitions: Vec::new(),
            dfa: DFA::new(),
            alphabets: BitSet::new(),
            accept: Vec::new(),
            deg: 0,
        }
    }
    pub fn construct(&mut self, dfa: &DFA) {
        self.dfa = dfa.clone();
        let ident = Matrix::Ident(dfa.size());
        self.transitions_map.insert(ident.clone(), 0);
        let mut queue = VecDeque::new();
        queue.push_back(ident);
        println!("dfa size:{}", dfa.size());
        while !queue.is_empty() {
            let mat = queue.front().unwrap().clone();
            for c in 0..256u32 {
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
                    queue.push_back(next);
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
                if mat_i.0.get(0, i) != 0 && dfa.states[i].accept {
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
    pub fn morphism(&self, s: &String) -> ElemType {
        let chars: Vec<char> = s.chars().collect();
        for state in 0..self.dfa.size() {
            for ch in &chars {}
        }
        0
    }

    pub fn starfree_expression(&self) -> Option<RegularExpression> {
        let memo: HashMap<ElemType, String> = HashMap::new();
        None
    }
    fn starfree_recursion(&self, e: ElemType, memo: &mut HashMap<ElemType, String>) -> String {
        if let Some(ref s) = memo.get(&e) {
            return s.to_string();
        }
        let regex = if identity(&e) {
            let W = [false; 256];
            for c in 0..256 {}

            String::new()
        } else {
            let AWA = String::new();
            let UA = String::new();
            let AV = String::new();
            format!("!(!({})|!({})|{})", UA, AV, AWA)
        };
        memo.insert(e, regex.to_string());
        regex
    }
}
