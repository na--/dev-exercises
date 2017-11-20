extern crate solution;
use solution::Polynomial;

#[test]
fn test_simple_functions() {
    assert_eq!(Polynomial::from(vec![]), Polynomial::from(vec![]));
    assert_eq!(Polynomial::from(vec![]), Polynomial::from(vec![0.0]));
    assert_eq!(Polynomial::from(vec![]), Polynomial::from(vec![0.0, 0.0]));
    assert_eq!(Polynomial::from(vec![0.0, 1.0]), Polynomial::from(vec![1.0]));
    assert_eq!(
        Polynomial::from(vec![1.0, 2.0, 3.0]),
        Polynomial::from(vec![0.0, 0.0, 1.0, 2.0, 3.0])
    );

    let p = Polynomial::from(vec![2.0, 0.0, 1.0, 7.0]);
    assert!(p.has(&(0.0, 7.0)));
    assert!(! p.has(&(-1.0, 1.0)));

    assert_eq!(
        Polynomial::from(vec![1.0, 2.0, 3.0]) * 2.0,
        Polynomial::from(vec![2.0, 4.0, 6.0])
    );
    assert_eq!(
        Polynomial::from(vec![1.0, 2.0, 3.0]) * 0.0,
        Polynomial::from(vec![])
    );

    assert_eq!(p.clone()/1.0, p);
    assert_eq!(
        Polynomial::from(vec![1.0, 2.0, 3.0]) / 2.0 ,
        Polynomial::from(vec![0.5, 1.0, 1.5])
    );
}


#[test]
fn test_addition_multiplication() {
    assert_eq!(
        Polynomial::from(vec![1.0, 2.0]) + Polynomial::from(vec![-1.0, -2.0]),
        Polynomial::from(vec![0.0])
    );

    assert_eq!(
        Polynomial::from(vec![1.0, 2.0, 3.0]) + Polynomial::from(vec![4.0, 5.0]),
        Polynomial::from(vec![1.0, 6.0, 8.0])
    );

    assert_eq!(
        Polynomial::from(vec![1.0, 2.0]) * Polynomial::from(vec![2.0, 1.0]),
        Polynomial::from(vec![2.0, 5.0, 2.0])
    );

    assert_eq!(
        Polynomial::from(vec![1.0, 2.0]) * Polynomial::from(vec![0.0]),
        Polynomial::from(vec![0.0])
    );
}

#[test]
fn test_mutability() {
    let p = Polynomial::from(vec![1.0, 2.0, 3.0, 4.0]);
    let q = Polynomial::from(vec![5.0, 6.0]);

    assert_eq!(
        (p.clone() * 2.0)/2.0,
        p
    );

    assert_eq!(
        p.clone() + q.clone(),
        Polynomial::from(vec![1.0, 2.0, 8.0, 10.0])
    );
/*
    assert_eq!(
        p * q,
        Polynomial::from(vec![1.0, 2.0, 8.0, 10.0])
    );
*/
}

#[test]
fn test_interpolation() {
    assert_eq!(
        Polynomial::interpolate(vec! {
            ( 1.0, 1.0),
            ( 1.0, 4.0),
            ( 3.0, 9.0),
        }),
        None
    );
    assert_eq!(
        Polynomial::interpolate(vec! {
            ( 1.0, 1.0),
            ( 2.0, 4.0),
            ( 3.0, 9.0),
        }).unwrap(),
        Polynomial::from(vec![1.0, 0.0, 0.0])
    );
    assert_eq!(
        Polynomial::interpolate(vec! {
            ( 1.0, 1.0),
            ( 2.0, 8.0),
            ( 3.0, 27.0),
        }).unwrap(),
        Polynomial::from(vec![6.0, -11.0, 6.0])
    );
}