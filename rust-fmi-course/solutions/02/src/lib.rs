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
        eq(point.1, self.coefs.iter().enumerate().fold(0.0, |sum, v| {
            sum + v.1 * f64::powi(point.0, v.0 as i32)
        }))
    }

    pub fn interpolate(points: Vec<(f64, f64)>) -> Option<Self> {
        let mut res = Polynomial::default();

        for (j, jpoint) in points.iter().enumerate() {
            let mut interm = Polynomial::new(vec![jpoint.1]);
            for (m, mpoint) in points.iter().enumerate() {
                if j == m {
                    continue;
                }
                if jpoint.0 == mpoint.0  {
                    return None;
                }
                interm = interm
                    * Polynomial::new(vec![-mpoint.0, 1.0])
                    / (jpoint.0 - mpoint.0);

            };
            res = res + interm;
        };

        Some(res)
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
        Polynomial::new(self.coefs.iter().map(|&c| c*rhs).collect())
    }
}

impl Div<f64> for Polynomial {
    type Output = Polynomial;
    fn div(self, rhs: f64) -> Self::Output {
        Polynomial::new(self.coefs.iter().map(|&c| c/rhs).collect())
    }
}

impl Mul for Polynomial {
    type Output = Polynomial;
    fn mul(self, rhs: Polynomial) -> Self::Output{
        let mut res = vec![0.0; self.coefs.len()+rhs.coefs.len()];
        for (li, litem) in self.coefs.iter().enumerate() {
            for (ri, ritem) in rhs.coefs.iter().enumerate() {
                res[li+ri] = res[li+ri] + litem*ritem
            }
        }
        Polynomial::new(res)
    }
}

impl Add for Polynomial {
    type Output = Polynomial;
    fn add(self, rhs: Polynomial) -> Self::Output{
        let (mut long,short) = if self.coefs.len()>rhs.coefs.len() {
            (self,rhs)
        } else {
            (rhs, self)
        };

        for (i, item) in short.coefs.iter().enumerate() {
            long.coefs[i] = long.coefs[i] + item;
        }
        long.trim_trailing_zeros();
        long
    }
}
