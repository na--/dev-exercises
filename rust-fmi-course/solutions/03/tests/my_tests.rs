extern crate solution;
use solution::*;

macro_rules! assert_match {
    ($pattern:pat, $actual:expr) => {
        if let $pattern = $actual {
            assert!(true);
        } else {
            assert!(false, "Expected {} to match {:?}", stringify!($pattern), $actual);
        }
    }
}

#[test]
fn command_test() {
    assert_match!(Ok(Command::Help), "h".parse::<Command>());
    assert_match!(Ok(Command::Info), "i".parse::<Command>());
    assert_match!(Ok(Command::Info), "in".parse::<Command>());
    assert_match!(Ok(Command::Quit), "q".parse::<Command>());
    assert_match!(Ok(Command::Quit), "qu".parse::<Command>());
    assert_match!(
        Ok(Command::TryLetter('x')),
        "try letter x".parse::<Command>()
    );
    assert_match!(Ok(Command::TryWord(_)), "try word xyzzy".parse::<Command>());
    assert_match!(Ok(Command::TryLetter('x')), "t l x".parse::<Command>());


    assert_match!(Err(GameError::ParseError(_)), "z".parse::<Command>());
}
