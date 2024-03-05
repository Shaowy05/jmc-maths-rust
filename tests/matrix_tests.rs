mod test_data;

use num_rational::Rational64;
use test_data::Rational;
use jmc_maths_rust::matrix::Matrix;
use jmc_maths_rust::vector::Vector;
// use jmc_maths_rust::field::Field;

#[test]
fn matrix_creation_test() {
    #[allow(unused_variables)]
    let m1 = Matrix::new_from_column_vectors(
        [
            Vector::new(
                [
                    Rational::new(Rational64::new(1, -2)),
                    Rational::new(Rational64::new(3, 4)),
                    Rational::new(Rational64::new(-5, 3)),
                ]
            ),
            Vector::new(
                [
                    Rational::new(Rational64::new(2, -7)),
                    Rational::new(Rational64::new(-9, -2)),
                    Rational::new(Rational64::new(6, 5)),
                ]
            ),
        ]
    );

    #[allow(unused_variables)]
    let m2 = Matrix::new_from_row_vectors(
        [
            Vector::new(
                [
                    Rational::new(Rational64::new(1, -2)),
                    Rational::new(Rational64::new(3, 4)),
                    Rational::new(Rational64::new(-5, 3)),
                ]
            ),
            Vector::new(
                [
                    Rational::new(Rational64::new(2, -7)),
                    Rational::new(Rational64::new(-9, -2)),
                    Rational::new(Rational64::new(6, 5)),
                ]
            ),
        ]
    );

    #[allow(unused_variables)]
    let m3 = Matrix::new(
        [
            [
                Rational::new(Rational64::new(1, -2)),
                Rational::new(Rational64::new(3, 4)),
                Rational::new(Rational64::new(-5, 3)),
            ],
            [
                Rational::new(Rational64::new(2, -7)),
                Rational::new(Rational64::new(-9, -2)),
                Rational::new(Rational64::new(6, 5)),
            ],
        ]
    );

    // No error thrown means the test passed
}

#[test]
fn matrix_index_test() {
    let m = Matrix::new(
        [
            [
                Rational::new(Rational64::new(1, -2)),
                Rational::new(Rational64::new(3, 4)),
                Rational::new(Rational64::new(-5, 3)),
            ],
            [
                Rational::new(Rational64::new(2, -7)),
                Rational::new(Rational64::new(-9, -2)),
                Rational::new(Rational64::new(6, 5)),
            ],
        ]
    );

    assert_eq!(m[0][0], Rational::new(Rational64::new(1, -2)));
    assert_eq!(m[0][1], Rational::new(Rational64::new(3, 4)));
    assert_eq!(m[0][2], Rational::new(Rational64::new(-5, 3)));
    assert_eq!(m[1][0], Rational::new(Rational64::new(2, -7)));
    assert_eq!(m[1][1], Rational::new(Rational64::new(-9, -2)));
    assert_eq!(m[1][2], Rational::new(Rational64::new(6, 5)));
}

#[test]
fn matrix_print_test() {
    let m = Matrix::new(
        [
            [
                Rational::new(Rational64::new(1, -2)),
                Rational::new(Rational64::new(3, 4)),
                Rational::new(Rational64::new(-5, 3)),
            ],
            [
                Rational::new(Rational64::new(2, -7)),
                Rational::new(Rational64::new(-9, -2)),
                Rational::new(Rational64::new(6, 5)),
            ],
        ]
    );

    println!("{:?}", m);
}