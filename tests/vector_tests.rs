mod test_data;

use num_rational::Rational64;

use test_data::Rational;
use jmc_maths_rust::vector::Vector;

#[test]
fn vector_creation_test() {
    #[allow(unused_variables)]
    let v = Vector::new(
        [
            Rational::new(Rational64::new(1, -2)),
            Rational::new(Rational64::new(3, 4)),
            Rational::new(Rational64::new(-5, 3)),
            Rational::new(Rational64::new(2, -7)),
            Rational::new(Rational64::new(-9, -2)),
            Rational::new(Rational64::new(6, 5)),
        ]
    );

    // No error thrown means the test passed
}
