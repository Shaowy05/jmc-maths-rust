use jmc_maths_rust::field::Field;
use std::ops::{Add, Div, Mul, Neg, Sub};

use num_rational::Rational64;

#[derive(Debug, Clone)]
pub struct Rational {
    value: Rational64,
}

impl Rational {
    pub fn new(value: Rational64) -> Rational {
        Rational { value }
    }
}

impl Field for Rational {
    fn additive_identity() -> Self {
        Rational {
            value: Rational64::from_integer(0),
        }
    }

    fn multiplicative_identity() -> Self {
        Rational {
            value: Rational64::from_integer(1),
        }
    }

    fn additive_inverse(&self) -> Self {
        Rational {
            value: Rational64::from_integer(0) - self.value,
        }
    }

    fn multiplicative_inverse(&self) -> Self {
        if *self == Rational::multiplicative_identity() {
            panic!("Division by zero");
        }

        Rational {
            value: Rational64::from_integer(1) / self.value,
        }
    }
}

impl Add for Rational {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Rational {
            value: self.value + other.value,
        }
    }
}

impl Sub for Rational {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Rational {
            value: self.value - other.value,
        }
    }
}

impl Mul for Rational {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Rational {
            value: self.value * other.value,
        }
    }
}

impl Div for Rational {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Rational {
            value: self.value / other.value,
        }
    }
}

impl Neg for Rational {
    type Output = Self;

    fn neg(self) -> Self {
        Rational { value: -self.value }
    }
}

impl PartialEq for Rational {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Copy for Rational {}
