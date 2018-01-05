extern crate solution;
use solution::*;

#[test]
fn test_empty() {
    let mut t = TrigramIterator::new("");
    assert_eq!(None, t.next());
}

#[test]
fn test_one_letter() {
    let mut t = TrigramIterator::new("c");
    assert_eq!(Some(String::from("  c")), t.next());
    assert_eq!(None, t.next());
}

#[test]
fn test_two_letters() {
    let mut t = TrigramIterator::new("ca");
    assert_eq!(Some(String::from("  c")), t.next());
    assert_eq!(Some(String::from(" ca")), t.next());
    assert_eq!(Some(String::from("ca ")), t.next());
    assert_eq!(None, t.next());
}

#[test]
fn test_three_letters() {
    let mut t = TrigramIterator::new("cat");
    assert_eq!(Some(String::from("  c")), t.next());
    assert_eq!(Some(String::from(" ca")), t.next());
    assert_eq!(Some(String::from("cat")), t.next());
    assert_eq!(Some(String::from("at ")), t.next());
    assert_eq!(None, t.next());
}

#[test]
fn test_four_letters() {
    let mut t = TrigramIterator::new("dogo");
    assert_eq!(Some(String::from("  d")), t.next());
    assert_eq!(Some(String::from(" do")), t.next());
    assert_eq!(Some(String::from("dog")), t.next());
    assert_eq!(Some(String::from("ogo")), t.next());
    assert_eq!(Some(String::from("go ")), t.next());
    assert_eq!(None, t.next());
}
