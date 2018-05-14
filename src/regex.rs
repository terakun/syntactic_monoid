#[derive(Debug, Clone, PartialEq)]
pub enum RegularExpression {
    Empty,
    Epsilon,
    Char(u8), // only ASCII
    Union(Box<RegularExpression>, Box<RegularExpression>),
    Concat(Box<RegularExpression>, Box<RegularExpression>),
    Kleene(Box<RegularExpression>),
}

/*
 *
 * <union> ::= <concat>
 *                | <concat> "|" <union>
 * <concat> ::= <kleene>
 *          | <kleene> <concat>
 * <kleene> ::= <factor>
 *          |   <factor> "*"
 * <factor> ::= <alphabet> | "(" <expression> ")"
 *
 */

impl RegularExpression {
    pub fn to_string(&self) -> String {
        match *self {
            RegularExpression::Empty => "∅".to_string(),
            RegularExpression::Epsilon => "ε".to_string(),
            RegularExpression::Char(a) => (a as char).to_string(),
            RegularExpression::Concat(ref e1, ref e2) => e1.to_string() + &e2.to_string(),
            RegularExpression::Union(ref e1, ref e2) => {
                "(".to_string() + &e1.to_string() + "+" + &e2.to_string() + ")"
            }
            RegularExpression::Kleene(ref e) => match **e {
                RegularExpression::Concat(_, _) => "(".to_string() + &e.to_string() + ")*",
                _ => e.to_string() + "*",
            },
        }
    }
}

pub struct Parser {
    cur: usize,
    chars: Vec<char>,
}

impl Parser {
    pub fn new() -> Self {
        Parser {
            cur: 0,
            chars: Vec::new(),
        }
    }
    pub fn parse(&mut self, text: &String) -> Option<RegularExpression> {
        self.chars = text.chars().collect();
        self.chars.push('\0');
        self.cur = 0;
        self.read_union()
    }

    // ab(ab+ba*)*
    fn read_union(&mut self) -> Option<RegularExpression> {
        let mut expleft: Option<RegularExpression> = self.read_concat();
        while self.chars[self.cur] == '+' {
            println!("{}", self.chars[self.cur]);
            self.cur = self.cur + 1;
            let expright = match self.read_concat() {
                Some(term) => term,
                None => {
                    return None;
                }
            };
            let el = expleft.unwrap().clone();
            expleft = Some(RegularExpression::Union(Box::new(el), Box::new(expright)));
        }
        expleft
    }

    fn read_concat(&mut self) -> Option<RegularExpression> {
        let mut expleft: Option<RegularExpression> = self.read_kleene();
        while self.chars[self.cur] != '\0' && self.chars[self.cur] != ')'
            && self.chars[self.cur] != '+'
        {
            let expright = match self.read_kleene() {
                Some(term) => term,
                None => {
                    return None;
                }
            };

            let el = expleft.unwrap().clone();
            expleft = Some(RegularExpression::Concat(Box::new(el), Box::new(expright)));
        }
        expleft
    }
    fn read_kleene(&mut self) -> Option<RegularExpression> {
        match self.read_factor() {
            Some(exp) => {
                let ch = self.chars[self.cur];
                if ch == '*' {
                    self.cur = self.cur + 1;
                    Some(RegularExpression::Kleene(Box::new(exp)))
                } else {
                    Some(exp)
                }
            }
            None => None,
        }
    }
    fn read_factor(&mut self) -> Option<RegularExpression> {
        let ch = self.chars[self.cur];
        match ch {
            '(' => {
                self.cur = self.cur + 1;
                let exp = match &self.read_union() {
                    &Some(ref exp) => exp.clone(),
                    &None => {
                        return None;
                    }
                };
                if self.chars[self.cur] != ')' {
                    println!("unterminated (");
                    return None;
                }
                self.cur = self.cur + 1;
                Some(exp)
            }
            ')' => None,
            '\0' => None,
            ch => {
                self.cur = self.cur + 1;
                Some(RegularExpression::Char(ch as u8))
            }
        }
    }
}
