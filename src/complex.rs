use std::{
    fmt::{Debug, Display, Formatter},
    ops
};

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Complex {
    re: f64,
    im: f64
}


impl Complex {
    // Static; Returns a new complex number
    pub fn new(re: f64, im: f64) -> Self {
        Complex { re, im }
    }

    // Static; Returns a complex number with equal real and imaginary parts
    pub fn uniform(x: f64) -> Self {
        Complex { re: x, im: x }
    }

    // Static; Returns complex number (0 + 0i)
    pub fn zero() -> Self {
        Complex::uniform(0.0)
    }

    // Non-Static; Returns the absolute value of itself
    pub fn abs(self) -> Complex {
        Complex::new(self.re.abs(), self.im.abs())
    }
}


// Addition Operator
impl ops::Add<Complex> for Complex {
    type Output = Complex;

    fn add(self, rhs: Self) -> Self::Output {
        Complex::new(self.re + rhs.re, self.im + rhs.im)
    }
}


// Multiplication operator, actually a cross product
impl ops::Mul<Complex> for Complex {
    type Output = Complex;

    fn mul(self, rhs: Complex) -> Self::Output {
        Complex::new(
            self.re * rhs.re - self.im * rhs.im,
            self.re * rhs.im + self.im * rhs.re,
        )
    }
}


impl Clone for Complex {
    fn clone(&self) -> Self {
        *self
    }
}


impl Copy for Complex {

}


impl Display for Complex {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        let sign = if self.im.is_sign_positive() {"+"} else {"-"};
        print!("{} {} {}i", self.re, sign, self.im.abs());
        Ok(())
    }
}