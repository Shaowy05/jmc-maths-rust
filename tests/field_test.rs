mod test_data;

use num_rational::Rational64;

use jmc_maths_rust::field::Field;
use test_data::TestField;

#[test]
fn field_add_test() {
    let a = TestField::new(Rational64::from_integer(1));
    let b = TestField::new(Rational64::new(2, 1));
    assert_eq!(a + b, TestField::new(Rational64::new(3, 1)));
}

#[test]
fn field_sub_test() {
    let a = TestField::new(Rational64::new(5, 1));
    let b = TestField::new(Rational64::new(9, 1));
    assert_eq!(a - b, TestField::new(Rational64::new(-4, 1)));
}

#[test]
fn field_mul_test() {
    let a = TestField::new(Rational64::new(3, 1));
    let b = TestField::new(Rational64::new(4, 1));
    assert_eq!(a * b, TestField::new(Rational64::new(12, 1)));
}

#[test]
fn field_div_test() {
    let a = TestField::new(Rational64::new(10, 1));
    let b = TestField::new(Rational64::new(5, 1));
    assert_eq!(a / b, TestField::new(Rational64::new(2, 1)));
}

#[test]
fn field_associative_addition_test() {
    let a = TestField::new(Rational64::new(8, 1));
    let a_ = a.clone();
    let b = TestField::new(Rational64::new(-2, 1));
    let b_ = b.clone();
    let c = TestField::new(Rational64::new(3, 1));
    let c_ = c.clone();
    assert_eq!((a + b) + c, a_ + (b_ + c_));
}

#[test]
fn field_associative_multiplication_test() {
    let a = TestField::new(Rational64::new(8, 1));
    let a_ = a.clone();
    let b = TestField::new(Rational64::new(-2, 1));
    let b_ = b.clone();
    let c = TestField::new(Rational64::new(3, 1));
    let c_ = c.clone();
    assert_eq!((a * b) * c, a_ * (b_ * c_));
}

#[test]
fn field_distributive_test() {
    let a = TestField::new(Rational64::new(8, 1));
    let a_ = a.clone();
    let b = TestField::new(Rational64::new(-2, 1));
    let b_ = b.clone();
    let c = TestField::new(Rational64::new(3, 1));
    let c_ = c.clone();
    assert_eq!(a * (b + c), (a_ * b_) + (a_ * c_));
}

#[test]
fn field_commutative_addition_test() {
    let a = TestField::new(Rational64::new(8, 1));
    let a_ = a.clone();
    let b = TestField::new(Rational64::new(-2, 1));
    let b_ = b.clone();
    assert_eq!(a + b, b_ + a_);
}

#[test]
fn field_commutative_multiplication_test() {
    let a = TestField::new(Rational64::new(8, 1));
    let a_ = a.clone();
    let b = TestField::new(Rational64::new(-2, 1));
    let b_ = b.clone();
    assert_eq!(a * b, b_ * a_);
}

#[test]
fn field_inverse_addition_test() {
    let a = TestField::new(Rational64::new(8, 1));
    let zero = TestField::new(Rational64::new(0, 1));
    assert_eq!(a + TestField::additive_inverse(&a), zero);
}

#[test]
fn field_inverse_multiplication_test() {
    let a = TestField::new(Rational64::new(8, 1));
    let one = TestField::new(Rational64::new(1, 1));
    assert_eq!(a * TestField::multiplicative_inverse(&a), one);
}

#[test]
fn field_multiplicative_identity_test() {
    let a = TestField::new(Rational64::new(8, 1));
    let a_ = a.clone();
    assert_eq!(a * TestField::multiplicative_identity(), a_);
}

#[test]
fn field_additive_identity_test() {
    let a = TestField::new(Rational64::new(8, 1));
    let a_ = a.clone();
    assert_eq!(a + TestField::additive_identity(), a_);
}
