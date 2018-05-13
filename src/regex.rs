#[derive(Debug, Clone)]
pub enum RegularExpression {
    Char(u8), // only ASCII
    Union(Box<RegularExpression>, Box<RegularExpression>),
    Concat(Box<RegularExpression>, Box<RegularExpression>),
    Kleene(Box<RegularExpression>),
}

/*
 * "a(ba)*b"
 */

impl RegularExpression {
    fn parse(text: &String) -> Option<Self> {
        let input: Vec<u8> = text.chars().map(|c| c as u8).collect();
        None
    }
}
