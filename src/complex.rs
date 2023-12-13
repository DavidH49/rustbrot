use std::{
    fmt::{Debug, Display, Formatter},
    ops
};

#[derive(PartialEq, PartialOrd)]
pub struct Complex {
    re: f64,
    im: f64
}


impl Complex {
    // Static functions
    pub fn new(re: f64, im: f64) -> Self {
        Complex { re, im }
    }

    pub fn uniform(x: f64) -> Self {
        Complex { re: x, im: x }
    }

    pub fn zero() -> Self {
        Complex::uniform(0.0)
    }

    // Non-Static functions
    pub fn abs(self) -> Complex {
        Complex::new(self.re.abs(), self.im.abs())
    }

    pub fn magnitude(self) -> f64 {
        (self.re * self.re + self.im * self.im).sqrt()
    }
}


// Operators
impl ops::Add<Complex> for Complex {
    type Output = Complex;

    fn add(self, rhs: Self) -> Self::Output {
        Complex::new(self.re + rhs.re, self.im + rhs.im)
    }
}


impl ops::Sub<Complex> for Complex {
    type Output = Complex;

    fn sub(self, rhs: Complex) -> Self::Output {
        Complex::new(self.re - rhs.re, self.im - rhs.im)
    }
}


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
        display_complex(self)?;
        Ok(())
    }
}


impl Debug for Complex {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        display_complex(self)?;
        Ok(())
    }
}


pub fn display_complex(c: & Complex) -> std::fmt::Result {
    let sign = if c.im.is_sign_positive() {"+"} else {"-"};
    print!("{} {} {}i", &c.re, sign, &c.im.abs());
    Ok(())
}