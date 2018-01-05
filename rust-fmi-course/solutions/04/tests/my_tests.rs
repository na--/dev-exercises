extern crate solution;
use std::collections::HashSet;
use solution::*;

macro_rules! set {
    ($($item:expr),*) => {
        {
            let mut hash_set = ::std::collections::HashSet::new();
            $( hash_set.insert($item); );*
            hash_set
        }
    };
}

#[test]
fn test_extract_words() {
    assert_eq!(extract_words(""), Vec::<String>::new());
    assert_eq!(extract_words(" "), Vec::<String>::new());
    assert_eq!(extract_words("   !!!  "), Vec::<String>::new());
    assert_eq!(extract_words("test"), vec!["test"]);
    assert_eq!(extract_words("  mest!!"), vec!["mest"]);
    assert_eq!(extract_words("zest   "), vec!["zest"]);
    assert_eq!(extract_words(" a! b? c&@#$&"), vec!["a", "b", "c"]);
    assert_eq!(
        extract_words("тест utf8 こんにちは!"),
        vec!["тест", "utf", "こんにちは"]
    );
}

#[test]
fn test_search() {
    let mut index = TextIndex::new();
    let empty = HashSet::new();
    assert_eq!(empty, index.search("something"));

    index.push("one, two");
    assert_eq!(set!{"one, two"}, index.search("two"));
    assert_eq!(empty, index.search("three"));
    index.push("two again!");
    assert_eq!(set!{"one, two", "two again!"}, index.search("two"));
    assert_eq!(set!{"two again!"}, index.search("again"));
    assert_eq!(empty, index.search("three"));
}
