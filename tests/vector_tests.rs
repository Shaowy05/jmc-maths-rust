mod test_data;

use test_data::Rational;
use jmc_maths_rust::vector::Vector;

#[test]
fn vector_creation_test() {
    #[allow(unused_variables)]
    let v = Vector::new(
        [
            Rational::new_from_args(1, -2),
            Rational::new_from_args(3, 4),
            Rational::new_from_args(-5, 3),
            Rational::new_from_args(2, -7),
            Rational::new_from_args(-9, -2),
            Rational::new_from_args(6, 5),
        ]
    );

    // No error thrown means the test passed
}

#[test]
fn vector_size_test() {
    assert_eq!(Vector::<Rational, 3>::dimension(), 3);
    assert_eq!(Vector::<Rational, 2123>::dimension(), 2123);

    let v_14 = Vector::new(
        [
            Rational::new_from_args(1, -2),
            Rational::new_from_args(3, 4),
            Rational::new_from_args(-5, 3),
            Rational::new_from_args(1, -2),
            Rational::new_from_args(3, 4),
            Rational::new_from_args(-5, 3),
            Rational::new_from_args(1, -2),
            Rational::new_from_args(3, 4),
            Rational::new_from_args(-5, 3),
            Rational::new_from_args(1, -2),
            Rational::new_from_args(3, 4),
            Rational::new_from_args(-5, 3),
            Rational::new_from_args(1, -2),
            Rational::new_from_args(3, 4),
        ]
    );
    assert_eq!(v_14.size(), 14);
}

#[test]
fn vector_as_array_test() {
    let v = Vector::new(
        [
            Rational::new_from_args(1, -2),
            Rational::new_from_args(3, 4),
            Rational::new_from_args(-5, 3),
        ]
    );

    assert_eq!(
        v.as_array(),
        [
            Rational::new_from_args(1, -2),
            Rational::new_from_args(3, 4),
            Rational::new_from_args(-5, 3),
        ]
    );

}

#[test]
fn vector_index_test() {
    let v = Vector::new(
        [
            Rational::new_from_args(1, -2),
            Rational::new_from_args(3, 4),
            Rational::new_from_args(-5, 3),
        ]
    );

    assert_eq!(v[0], Rational::new_from_args(1, -2));
    assert_eq!(v[1], Rational::new_from_args(3, 4));
    assert_eq!(v[2], Rational::new_from_args(-5, 3));
}

#[test]
#[should_panic]
fn vector_fail_index_test() {
    let v = Vector::new(
        [
            Rational::new_from_args(1, -2),
            Rational::new_from_args(3, 4),
            Rational::new_from_args(-5, 3),
        ]
    );

    #[allow(unused_variables)]
    let _ = v[3];
}

#[test]
fn vector_addition_test() {
    let a = Vector::new(
        [
            Rational::new_from_args(1, -2),
            Rational::new_from_args(3, 4),
            Rational::new_from_args(-5, 3),
        ]
    );

    let b = Vector::new(
        [
            Rational::new_from_args(-2, 1),
            Rational::new_from_args(4, 3),
            Rational::new_from_args(3, -5),
        ]
    );

    assert_eq!(
        a + b,
        Vector::new(
            [
                Rational::new_from_args(-5, 2),
                Rational::new_from_args(25, 12),
                Rational::new_from_args(-34, 15),
            ]
        )
    );
}

#[test]
fn vector_subtraction_test() {
    let a = Vector::new(
        [
            Rational::new_from_args(1, -2),
            Rational::new_from_args(3, 4),
            Rational::new_from_args(-5, 3),
        ]
    );

    let b = Vector::new(
        [
            Rational::new_from_args(-2, 1),
            Rational::new_from_args(4, 3),
            Rational::new_from_args(3, -5),
        ]
    );

    assert_eq!(
        a - b,
        Vector::new(
            [
                Rational::new_from_args(3, 2),
                Rational::new_from_args(-7, 12),
                Rational::new_from_args(-16, 15),
            ]
        )
    );
}

#[test]
fn vector_scalar_multiplication_test() {
    let a = Vector::new(
        [
            Rational::new_from_args(1, -2),
            Rational::new_from_args(3, 4),
            Rational::new_from_args(-5, 3),
        ]
    );

    let b = Rational::new_from_args(-2, 1);

    assert_eq!(
        a * b,
        Vector::new(
            [
                Rational::new_from_args(1, 1),
                Rational::new_from_args(-3, 2),
                Rational::new_from_args(10, 3),
            ]
        )
    );

}

#[test]
fn vector_dot_product_test() {
    let a = Vector::new(
        [
            Rational::new_from_args(1, -2),
            Rational::new_from_args(3, 4),
            Rational::new_from_args(-5, 3),
        ]
    );

    let b = Vector::new(
        [
            Rational::new_from_args(-2, 1),
            Rational::new_from_args(4, 3),
            Rational::new_from_args(3, -5),
        ]
    );

    assert_eq!(
        a * b,
        Rational::new_from_args(3, 1)
    );
}

#[test]
fn vector_equality_test() {
    let a = Vector::new(
        [
            Rational::new_from_args(1, -2),
            Rational::new_from_args(3, 4),
            Rational::new_from_args(-5, 3),
        ]
    );

    let b = Vector::new(
        [
            Rational::new_from_args(1, -2),
            Rational::new_from_args(3, 4),
            Rational::new_from_args(-5, 3),
        ]
    );

    assert_eq!(a, b);
}
