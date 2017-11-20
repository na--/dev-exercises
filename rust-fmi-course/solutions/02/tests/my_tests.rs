extern crate solution;
use solution::Polynomial;

#[test]
fn test_description_constraints() {
    assert_eq!(Polynomial::from(vec![]), Polynomial::from(vec![0.0]));
    assert_eq!(Polynomial::from(vec![]), Polynomial::from(vec![0.0, 0.0]));
    assert_eq!(Polynomial::from(vec![0.0, 1.0]), Polynomial::from(vec![1.0]));
    assert_eq!(Polynomial::from(vec![1.0, 2.0, 3.0]), Polynomial::from(vec![0.0, 0.0, 1.0, 2.0, 3.0]));
}


#[test]
fn test_while_implementing() {
    let p = Polynomial::from(vec![2.0, 0.0, 1.0, 7.0]);
    assert!(p.has(&(0.0, 7.0)));
    assert!(! p.has(&(-1.0, 1.0)));
    /*
    println!("Result 01: {:?}", Polynomial::from(vec![]));
    println!("Result 02: {:?}", Polynomial::from(vec![0.0]));
    println!("Result 03: {:?}", Polynomial::from(vec![0.0, 0.0]));
    println!("Result 04: {:?}", Polynomial::from(vec![1.2, 0.0]));
    println!("Result 05: {:?}", Polynomial::from(vec![0.0, 1.3]));
    println!("Result 06: {:?}", Polynomial::from(vec![1.4]));
    println!("Result 07: {:?}", Polynomial::from(vec![0.0, 0.0, 1.0, 2.0, 3.0]));
    */
}