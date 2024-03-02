use jmc_maths_rust::field::Field;
use std::ops::{Add, Sub, Mul, Div};

#[derive(Debug)]
struct TestField {
    value: i32,
}

impl Field for TestField {
    fn additive_identity() -> Self {
        TestField { value: 0 }
    }

    fn multiplicative_identity() -> Self {
        TestField { value: 1 }
    }

    fn additive_inverse(&self) -> Self {
        TestField { value: -self.value }
    }

    fn multiplicative_inverse(&self) -> Self {
        match self.value {
            0 => panic!("Division by zero"),
            _ => TestField { value: 1 / self.value },
            
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

impl PartialEq for TestField {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Clone for TestField {
    fn clone(&self) -> Self {
        TestField {
            value: self.value,
        }
    }
}

#[test]
fn field_add_test() {
    let a = TestField { value: 1 };
    let b = TestField { value: 2 };
    assert_eq!(a + b, TestField { value: 3 });
}

#[test]
fn field_sub_test() {
    let a = TestField { value: 5 };
    let b = TestField { value: 9 };
    assert_eq!(a - b, TestField { value: -4 });
}

#[test]
fn field_mul_test() {
    let a = TestField { value: 3 };
    let b = TestField { value: 4 };
    assert_eq!(a * b, TestField { value: 12 });
}

#[test]
fn field_div_test() {
    let a = TestField { value: 10 };
    let b = TestField { value: 2 };
    assert_eq!(a / b, TestField { value: 5 });
}

#[test]
fn field_associative_addition_test() {
    let a = TestField { value: 8 };
    let a_ = a.clone();
    let b = TestField { value: -2 };
    let b_ = b.clone();
    let c = TestField { value: 3 };
    let c_ = c.clone();
    assert_eq!((a + b) + c, a_ + (b_ + c_));
}

#[test]
fn field_associative_multiplication_test() {
    let a = TestField { value: 8 };
    let a_ = a.clone();
    let b = TestField { value: -2 };
    let b_ = b.clone();
    let c = TestField { value: 3 };
    let c_ = c.clone();
    assert_eq!((a * b) * c, a_ * (b_ * c_));
}
