mod regex;
mod dfa;
mod syntactic_monoid;
use regex::Parser;
use dfa::DFA;
use dfa::State;
use syntactic_monoid::SyntacticMonoid;

#[test]
fn regex_test() {
    let mut parser = Parser::new();
    let re = parser.parse(&"a+a+b".to_string()).unwrap();
    assert_eq!(re.to_string(), "((a+a)+b)".to_string());
    let re = parser.parse(&"(ab+aa)".to_string()).unwrap();
    assert_eq!(re.to_string(), "(ab+aa)".to_string());
    let re = parser.parse(&"aa+bbb".to_string()).unwrap();
    assert_eq!(re.to_string(), "(aa+bbb)".to_string());
    let re = parser.parse(&"(a(a+b)*b)*b+b".to_string()).unwrap();
    assert_eq!(re.to_string(), "((a(a+b)*b)*b+b)".to_string());
    let re = parser.parse(&"ab+ba*".to_string()).unwrap();
    assert_eq!(re.to_string(), "(ab+ba*)".to_string());
    let re = parser.parse(&"ab+ba".to_string()).unwrap();
    assert_eq!(re.to_string(), "(ab+ba)".to_string());
}

fn main() {
    let mut dfa = DFA::new();
    let mut s_a = State::new(0, true);
    let mut s_b = State::new(1, false);
    let mut s_c = State::new(2, false);

    let a = 'a' as usize;
    let b = 'b' as usize;

    s_a.add_trans(0, a);
    s_a.add_trans(1, b);

    s_b.add_trans(0, a);
    s_b.add_trans(2, b);

    s_c.add_trans(2, a);
    s_c.add_trans(2, b);

    dfa.add_state(s_a);
    dfa.add_state(s_b);
    dfa.add_state(s_c);

    dfa.to_graphviz();

    let mut sm = SyntacticMonoid::new();
    sm.construct(&dfa);
}
