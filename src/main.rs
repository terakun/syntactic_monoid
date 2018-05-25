use std::env;

mod regex;
mod nfa;
mod dfa;
mod syntactic_monoid;
use regex::Parser;
use nfa::NFA;
use dfa::DFA;
use dfa::State;
use syntactic_monoid::SyntacticMonoid;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = if args.len() >= 2 {
        args[1].clone()
    } else {
        "(a|ba)*".to_string()
    };
    let mut parser = Parser::new();
    let re = parser.parse(&input).unwrap();
    let nfa = NFA::construct(&re);
    let dfa = DFA::construct_from_nfa(&nfa);
    // nfa.to_graphviz();
    // dfa.to_graphviz();
    let min_dfa = dfa.minimize();
    min_dfa.to_graphviz();
    let mut sm = SyntacticMonoid::new();
    sm.construct(&min_dfa, &input);
    match sm.starfree_expression() {
        Some(exp) => {
            println!("{}", exp);
        }
        None => {
            println!("the monoid is aperiodic");
        }
    }
}
