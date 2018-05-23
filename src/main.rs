mod regex;
mod nfa;
mod dfa;
mod syntactic_monoid;
use regex::Parser;
use nfa::NFA;
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
    let mut parser = Parser::new();
    let re = parser.parse(&"(a|ab|ba)*".to_string()).unwrap();
    // let re = parser.parse(&"a+c+b+a".to_string()).unwrap();
    // println!("{:?}", re);
    let nfa = NFA::construct(&re);
    nfa.to_graphviz();
}
