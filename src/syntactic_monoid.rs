use super::regex;
use super::dfa::DFA;

struct Matrix {
    m: Vec<u8>,
}

type ElemType = usize;
pub struct SyntacticMonoid {
    multiplication_table: Vec<ElemType>,
    dfa: DFA,
}

impl SyntacticMonoid {
    fn construct() -> Self {
        SyntacticMonoid {
            multiplication_table: Vec::new(),
            dfa: DFA { states: Vec::new() },
        }
    }
}
