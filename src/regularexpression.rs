#[derive(Debug, Clone)]
pub enum RegularExpression {
    Empty,
    Epsilon,
    Singleton(char),
    Concat(Box<RegularExpression>, Box<RegularExpression>),
    Or(Box<RegularExpression>, Box<RegularExpression>),
    Kleene(Box<RegularExpression>),
}

impl RegularExpression {
    pub fn to_string(&self) -> String {
        use RegularExpression::*;
        match *self {
            Empty => "∅".to_string(),
            Epsilon => "ε".to_string(),
            Singleton(a) => a.to_string(),
            Concat(ref e1, ref e2) => e1.to_string() + &e2.to_string(),
            Or(ref e1, ref e2) => "(".to_string() + &e1.to_string() + "+" + &e2.to_string() + ")",
            Kleene(ref e) => e.to_string() + "*",
        }
    }

    pub fn reduction(&self) -> Self {
        use RegularExpression::*;
        match *self {
            Concat(ref e1, ref e2) => match **e1 {
                Empty => Empty,
                Epsilon => *e2.clone(),
                _ => match **e2 {
                    Empty => Empty,
                    Epsilon => *e1.clone(),
                    _ => Concat(Box::new(e1.reduction()), Box::new(e2.reduction())),
                },
            },
            Or(ref e1, ref e2) => match **e1 {
                Empty => *e2.clone(),
                _ => match **e2 {
                    Empty => *e1.clone(),
                    _ => Or(Box::new(e1.reduction()), Box::new(e2.reduction())),
                },
            },
            Kleene(ref e) => Kleene(Box::new(e.reduction())),
            _ => self.clone(),
        }
    }

    pub fn nullable(&self) -> Self {
        use RegularExpression::*;
        match *self {
            Epsilon | Kleene(_) => Epsilon,
            Concat(ref e1, ref e2) => Concat(Box::new(e1.nullable()), Box::new(e2.nullable())),
            Or(ref e1, ref e2) => Or(Box::new(e1.nullable()), Box::new(e2.nullable())),
            _ => Empty,
        }
    }

    pub fn derivative(&self, a: char) -> Self {
        use RegularExpression::*;
        match *self {
            Empty => Empty,
            Epsilon => Epsilon,
            Singleton(c) => if c == a {
                Epsilon
            } else {
                Empty
            },
            Concat(ref e1, ref e2) => Or(
                Box::new(Concat(Box::new(e1.derivative(a)), Box::new(*e2.clone()))),
                Box::new(Concat(Box::new(e1.nullable()), Box::new(*e2.clone()))),
            ),
            Or(ref e1, ref e2) => Or(Box::new(e1.derivative(a)), Box::new(e2.derivative(a))),
            Kleene(ref e) => Concat(Box::new(e.derivative(a)), Box::new(Kleene(e.clone()))),
        }
    }
}
