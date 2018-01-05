use std::str::Chars;
use std::iter::Chain;

// Доста неоптимално решение, но ми се играеше с итератори...
pub struct TrigramIterator<'a> {
    c1: Chain<Chars<'a>, Chars<'a>>,
    c2: Chain<Chars<'a>, Chars<'a>>,
    c3: Chain<Chars<'a>, Chars<'a>>,
}

impl<'a> TrigramIterator<'a> {
    pub fn new(word: &'a str) -> Self {
        let tail = if word.chars().count() > 1 { " " } else { "" };
        TrigramIterator {
            c1: "  ".chars().chain(word.chars()),
            c2: " ".chars().chain(word.chars()),
            c3: word.chars().chain(tail.chars()),
        }
    }
}

impl<'a> Iterator for TrigramIterator<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        match (self.c1.next(), self.c2.next(), self.c3.next()) {
            (Some(c1), Some(c2), Some(c3)) => {
                let mut result = String::with_capacity(3);
                result.push(c1);
                result.push(c2);
                result.push(c3);
                Some(result)
            }
            _ => None,
        }
    }
}
