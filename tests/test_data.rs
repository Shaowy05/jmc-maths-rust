use jmc_maths_rust::field::Field;
use std::ops::{Add, Div, Mul, Neg, Sub};

use num_rational::Rational64;

#[derive(Debug, Clone)]
pub struct TestField {
    value: Rational64,
}

impl TestField {
    pub fn new(value: Rational64) -> TestField {
        TestField { value }
    }
}

impl Field for TestField {
    fn additive_identity() -> Self {
        TestField {
            value: Rational64::from_integer(0),
        }
    }

    fn multiplicative_identity() -> Self {
        TestField {
            value: Rational64::from_integer(1),
        }
    }

    fn additive_inverse(&self) -> Self {
        TestField {
            value: Rational64::from_integer(0) - self.value,
        }
    }

    fn multiplicative_inverse(&self) -> Self {
        if *self == TestField::multiplicative_identity() {
            panic!("Division by zero");
        }

        TestField {
            value: Rational64::from_integer(1) / self.value,
        }
    }
}

impl Add for TestField {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        TestField {
            value: self.value + other.value,
        }
    }
}

impl Sub for TestField {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        TestField {
            value: self.value - other.value,
        }
    }
}

impl Mul for TestField {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        TestField {
            value: self.value * other.value,
        }
    }
}

impl Div for TestField {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        TestField {
            value: self.value / other.value,
        }
    }
}

impl Neg for TestField {
    type Output = Self;

    fn neg(self) -> Self {
        TestField { value: -self.value }
    }
}

impl PartialEq for TestField {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Copy for TestField {}

// impl Clone for TestField {
//     fn clone(&self) -> Self {
//         TestField { value: self.value }
//     }
// }
