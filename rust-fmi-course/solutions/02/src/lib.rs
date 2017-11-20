use std::ops::{Mul,Div,Add};
use std::default::Default;
use std::cmp::PartialEq;

#[derive(Debug, Clone)]
pub struct Polynomial {
    coefs: Vec<f64>
}

fn eq(x: f64, y: f64) -> bool {
    // TODO: use someting more robust
    // https://users.rust-lang.org/t/assert-eq-for-float-numbers/7034/4
    (x - y).abs() < 1e-10
}

impl Polynomial {
    pub fn has(&self, point: &(f64, f64)) -> bool {
        //TODO: do this with iter.fold()?
        let mut val = 0.0;
        for (i, item) in self.coefs.iter().enumerate() {
            val = val + item * f64::powi(point.0, i as i32);
        }
        eq(val, point.1)
    }

    pub fn interpolate(points: Vec<(f64, f64)>) -> Option<Self> {
        // TODO
        None
    }

    fn trim_trailing_zeros(&mut self) {
        match self.coefs.iter().rposition(|&x| ! eq(x, 0.0)) {
            None => self.coefs.clear(),
            Some(pos) => self.coefs.truncate(pos+1),
        }
    }

    fn new(asc_coefs: Vec<f64>) -> Self {
        let mut p = Polynomial{coefs: asc_coefs};
        p.trim_trailing_zeros();
        return p
    }
}

impl From<Vec<f64>> for Polynomial {
    fn from(mut coefs: Vec<f64>) -> Self {
        coefs.reverse();
        return Polynomial::new(coefs)
    }
}

impl Default for Polynomial {
   fn default() -> Polynomial {
       Polynomial{coefs: vec![]}
    }
}

impl PartialEq for Polynomial {
    fn eq(&self, rhs: &Self) -> bool {
        if self.coefs.len() != rhs.coefs.len() {
            return false
        }
        for i in 0..self.coefs.len() {
            if ! eq(self.coefs[i], rhs.coefs[i]) {
                return false
            }
        }
        return true
    }
}

impl Mul<f64> for Polynomial {
    type Output = Polynomial;
    fn mul(self, rhs: f64) -> Self::Output {
        //TODO: figure out how to do this in-place, without creating a new vector...
        Polynomial::new(self.coefs.iter().map(|&c| c*rhs).collect())
    }
}

impl Div<f64> for Polynomial {
    type Output = Polynomial;
    fn div(self, rhs: f64) -> Self::Output {
        //TODO: figure out how to do this in-place, without creating a new vector...
        Polynomial::new(self.coefs.iter().map(|&c| c/rhs).collect())
    }
}

/*
impl Mul for Polynomial {
    type Output = Polynomial;
    fn mul(self, rhs: Polynomial) -> Self::Output{
        // TODO
        self
    }
}

impl Add for Polynomial {
    type Output = Polynomial;
    fn add(self, rhs: Polynomial) -> Self::Output{
        // TODO
        self
    }
}
*/