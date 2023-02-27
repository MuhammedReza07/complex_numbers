use std::{fmt, ops, f64};

#[derive(Clone, Copy)]
pub struct Complex {
    pub real: f64,
    pub imaginary: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.imaginary < 0.0 {
            write!(f, "{} - {}i", self.real, -self.imaginary)
        } else {
            write!(f, "{} + {}i", self.real, self.imaginary)
        }
    }
}

impl ops::Add for Complex {
    type Output = Complex;
    fn add(self, rhs: Complex) -> Complex {
        Complex {
            real: self.real + rhs.real,
            imaginary: self.imaginary + rhs.imaginary
        }
    }
}

impl ops::Mul for Complex {
    type Output = Complex;
    fn mul(self, rhs: Complex) -> Complex {
        Complex {
            real: (self.real * rhs.real) - (self.imaginary * rhs.imaginary),
            imaginary: (self.real * rhs.imaginary) + (self.imaginary * rhs.real)
        }
    }
}

impl Complex {
    pub fn new(real: f64, imaginary: f64) -> Complex {
        Complex {real: real, imaginary: imaginary}
    }
    pub fn abs(self) -> f64 {
        ((self.real * self.real) + (self.imaginary * self.imaginary)).sqrt()
    }
}