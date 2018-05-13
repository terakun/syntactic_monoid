#[derive(Debug, Clone)]
pub enum RegularExpression {
    Char(u8), // ASCII
    Union(Box<RegularExpression>, Box<RegularExpression>),
    Concat(Box<RegularExpression>, Box<RegularExpression>),
    Kleene(Box<RegularExpression>),
}

impl RegularExpression {
    fn parse(text: &String) -> Option<Self> {
        None
    }
}
