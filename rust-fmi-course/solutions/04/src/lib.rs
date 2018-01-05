use std::str::CharIndices;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

struct WordIterator<'a> {
    text: &'a str,
    iter: CharIndices<'a>,
    word_start: Option<usize>,
}

impl<'a> WordIterator<'a> {
    pub fn new(text: &'a str) -> Self {
        WordIterator {
            text: text,
            iter: text.char_indices(),
            word_start: None,
        }
    }
}

// This probably would have been much nicer if std::str::pattern::Pattern was stable...
impl<'a> Iterator for WordIterator<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match (self.word_start, self.iter.next()) {
                (None, Some((pos, c))) => {
                    if c.is_alphabetic() {
                        self.word_start = Some(pos);
                    }
                }
                (Some(pos), None) => {
                    self.word_start = None;
                    return Some(&self.text[pos..]);
                }
                (Some(start_pos), Some((end_pos, c))) => {
                    if !c.is_alphabetic() {
                        self.word_start = None;
                        return Some(&self.text[start_pos..end_pos]);
                    }
                }
                (None, None) => return None,
            }
        }
    }
}

pub fn extract_words(text: &str) -> Vec<String> {
    WordIterator::new(text).map(String::from).collect()
}



pub struct TextIndex {
    index: HashMap<String, HashSet<Rc<String>>>,
}

impl TextIndex {
    pub fn new() -> Self {
        TextIndex {
            index: HashMap::new(),
        }
    }

    pub fn push(&mut self, text: &str) {
        let rc_text = Rc::new(String::from(text));
        for word in extract_words(text) {
            let mut textset = self.index.entry(word).or_insert(HashSet::new());
            textset.insert(rc_text.clone());
        }
    }

    pub fn search(&self, query: &str) -> HashSet<&str> {
        let mut result = HashSet::new();
        for word in extract_words(query) {
            if let Some(textset) = self.index.get(&word) {
                for text in textset {
                    result.insert(text.as_ref().as_ref());
                }
            }
        }
        return result;
    }
}
