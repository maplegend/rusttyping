use orbtk::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct AttributedLetter {
    pub character: char,
    pub color: Foreground
}

impl AttributedLetter {
    pub fn new(character: char, color: Foreground) -> Self{
        AttributedLetter {character, color}
    }
}

property!(
    AttributedText(Vec<AttributedLetter>)
);

impl AttributedText{
    pub fn to_string(&self) -> String{
        self.0.iter().fold(String::new(), |mut rs, l| {rs.push(l.character); rs})
    }
}