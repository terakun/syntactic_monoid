mod regex;
mod dfa;
mod syntactic_monoid;
use regex::Parser;
use dfa::DFA;
use dfa::State;

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
    let mut s_i = State::new(0, false);
    let mut s_1 = State::new(1, false);
    let mut s_2 = State::new(2, false);
    let mut s_3 = State::new(3, false);
    let mut s_f = State::new(4, true);

    let a = 'a' as usize;
    let b = 'b' as usize;

    s_i.add_trans(1, a);

    s_1.add_trans(2, a);
    s_1.add_trans(3, b);

    s_2.add_trans(2, a);
    s_2.add_trans(3, b);

    s_3.add_trans(2, a);
    s_3.add_trans(4, b);

    s_f.add_trans(2, a);
    s_f.add_trans(4, b);

    dfa.add_state(s_i);
    dfa.add_state(s_1);
    dfa.add_state(s_2);
    dfa.add_state(s_3);
    dfa.add_state(s_f);

    dfa.to_graphviz();

    let vecu8: Vec<u8> = "aab".to_string().as_bytes().iter().map(|c| *c).collect();
    if dfa.accept(&vecu8) {
        println!("accept");
    } else {
        println!("reject");
    }
    let min_dfa = dfa.minimize();
    min_dfa.to_graphviz();
}
