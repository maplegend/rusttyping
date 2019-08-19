use orbtk::{prelude::*, utils::prelude::*};
use std::cmp::PartialEq;

#[derive(Debug, Clone)]
pub struct Letter{
    pub character: char,
    pub id: String
}

impl Letter{
    pub fn new(character: char, id: String) -> Letter{
        Letter{character, id}
    }
}

impl PartialEq for Letter {
    fn eq(&self, other: &Self) -> bool {
        self.character == other.character && self.id == other.id
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

property!(
    StyledText(Vec<Letter>)
);