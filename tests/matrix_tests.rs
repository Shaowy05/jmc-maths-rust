mod test_data;

use test_data::Rational;
use jmc_maths_rust::matrix::Matrix;

#[test]
fn matrix_transpose_test() {
    let m = Matrix::new(
        [
            [
                Rational::new_from_args(1, -2),
                Rational::new_from_args(3, 4),
                Rational::new_from_args(-5, 3),
            ],
            [
                Rational::new_from_args(2, -7),
                Rational::new_from_args(-9, -2),
                Rational::new_from_args(6, 5),
            ],
        ]
    );

    let m_t = m.transpose();

    assert_eq!(m_t[0][0], Rational::new_from_args(1, -2));
    assert_eq!(m_t[0][1], Rational::new_from_args(2, -7));
    assert_eq!(m_t[1][0], Rational::new_from_args(3, 4));
    assert_eq!(m_t[1][1], Rational::new_from_args(-9, -2));
    assert_eq!(m_t[2][0], Rational::new_from_args(-5, 3));
    assert_eq!(m_t[2][1], Rational::new_from_args(6, 5));
}

#[test]
fn matrix_row_and_column_test() {
    let m = Matrix::new(
        [
            [
                Rational::new_from_args(1, -2),
                Rational::new_from_args(3, 4),
                Rational::new_from_args(-5, 3),
            ],
            [
                Rational::new_from_args(2, -7),
                Rational::new_from_args(-9, -2),
                Rational::new_from_args(6, 5),
            ],
        ]
    );

    let r1 = m.row(0);
    let r2 = m.row(1);
    let c1 = m.column(0);
    let c2 = m.column(1);

    assert_eq!(r1[0], Rational::new_from_args(1, -2));
    assert_eq!(r1[1], Rational::new_from_args(-2, 7));
    assert_eq!(r2[0], Rational::new_from_args(3, 4));
    assert_eq!(r2[1], Rational::new_from_args(-9, -2));
    assert_eq!(c1[0], Rational::new_from_args(1, -2));
    assert_eq!(c1[1], Rational::new_from_args(3, 4));
    assert_eq!(c1[2], Rational::new_from_args(-5, 3));
    assert_eq!(c2[0], Rational::new_from_args(2, -7));
    assert_eq!(c2[1], Rational::new_from_args(9, 2));
    assert_eq!(c2[2], Rational::new_from_args(6, 5));

}

#[test]
fn matrix_is_square_test() {
    let m1 = Matrix::new(
        [
            [
                Rational::new_from_args(1, -2),
                Rational::new_from_args(3, 4),
                Rational::new_from_args(-5, 3),
            ],
            [
                Rational::new_from_args(2, -7),
                Rational::new_from_args(-9, -2),
                Rational::new_from_args(6, 5),
            ],
            [
                Rational::new_from_args(2, -7),
                Rational::new_from_args(-9, -2),
                Rational::new_from_args(6, 5),
            ],
        ]
    );

    let m2 = Matrix::new(
        [
            [
                Rational::new_from_args(1, -2),
                Rational::new_from_args(3, 4),
            ],
            [
                Rational::new_from_args(2, -7),
                Rational::new_from_args(-9, -2),
            ],
        ]
    );

    let m3 = Matrix::new(
        [
            [
                Rational::new_from_args(1, -2),
                Rational::new_from_args(3, 4),
                Rational::new_from_args(-5, 3),
            ],
            [
                Rational::new_from_args(2, -7),
                Rational::new_from_args(-9, -2),
                Rational::new_from_args(6, 5),
            ],
        ]
    );

    assert_eq!(m1.is_square(), true);
    assert_eq!(m2.is_square(), true);
    assert_eq!(m3.is_square(), false);
}

#[test]
fn matrix_is_upper_triangular_test() {
    let m1 = Matrix::new(
        [
            [
                Rational::new_from_args(1, -2),
                Rational::new_from_args(0, 4),
                Rational::new_from_args(0, 3),
            ],
            [
                Rational::new_from_args(4, 1),
                Rational::new_from_args(0, -2),
                Rational::new_from_args(0, 5),
            ],
            [
                Rational::new_from_args(3, 1),
                Rational::new_from_args(4, -2),
                Rational::new_from_args(2, 5),
            ],
        ]
    );

    let m2 = Matrix::new(
        [
            [
                Rational::new_from_args(1, -2),
                Rational::new_from_args(3, 4),
                Rational::new_from_args(-5, 3),
            ],
            [
                Rational::new_from_args(2, -7),
                Rational::new_from_args(-9, -2),
                Rational::new_from_args(6, 5),
            ],
        ]
    );

    let m3 = Matrix::new(
        [
            [
                Rational::new_from_args(1, -2),
                Rational::new_from_args(0, 4),
                Rational::new_from_args(3, 3),
            ],
            [
                Rational::new_from_args(4, 1),
                Rational::new_from_args(0, -2),
                Rational::new_from_args(0, 5),
            ],
            [
                Rational::new_from_args(3, 1),
                Rational::new_from_args(0, -2),
                Rational::new_from_args(0, 5),
            ],
        ]
    );

    assert_eq!(m1.is_upper_triangular(), true);
    assert_eq!(m2.is_upper_triangular(), false);
    assert_eq!(m3.is_upper_triangular(), false);

}

#[test]
fn matrix_is_strictly_upper_triangular_test() {
    let m1 = Matrix::new(
        [
            [
                Rational::new_from_args(0, -2),
                Rational::new_from_args(0, 4),
                Rational::new_from_args(0, 3),
            ],
            [
                Rational::new_from_args(4, 1),
                Rational::new_from_args(0, -2),
                Rational::new_from_args(0, 5),
            ],
            [
                Rational::new_from_args(3, 1),
                Rational::new_from_args(4, -2),
                Rational::new_from_args(0, 2),
            ],
        ]
    );

    let m2 = Matrix::new(
        [
            [
                Rational::new_from_args(1, -2),
                Rational::new_from_args(3, 4),
                Rational::new_from_args(-5, 3),
            ],
            [
                Rational::new_from_args(2, -7),
                Rational::new_from_args(-9, -2),
                Rational::new_from_args(6, 5),
            ],
        ]
    );

    let m3 = Matrix::new(
        [
            [
                Rational::new_from_args(1, -2),
                Rational::new_from_args(0, 4),
                Rational::new_from_args(0, 3),
            ],
            [
                Rational::new_from_args(4, 1),
                Rational::new_from_args(0, -2),
                Rational::new_from_args(0, 5),
            ],
            [
                Rational::new_from_args(3, 1),
                Rational::new_from_args(0, -2),
                Rational::new_from_args(0, 5),
            ],
        ]
    );

    assert_eq!(m1.is_strictly_upper_triangular(), true);
    assert_eq!(m2.is_strictly_upper_triangular(), false);
    assert_eq!(m3.is_strictly_upper_triangular(), false);

}

#[test]
fn matrix_is_lower_triangular_test() {
    let m1 = Matrix::new(
        [
            [
                Rational::new_from_args(0, 2),
                Rational::new_from_args(3, 1),
                Rational::new_from_args(4, -2),
            ],
            [
                Rational::new_from_args(0, -2),
                Rational::new_from_args(5, 4),
                Rational::new_from_args(4, 3),
            ],
            [
                Rational::new_from_args(0, -2),
                Rational::new_from_args(0, 5),
                Rational::new_from_args(4, 1),
            ],
        ]
    );

    let m2 = Matrix::new(
        [
            [
                Rational::new_from_args(1, -2),
                Rational::new_from_args(3, 4),
                Rational::new_from_args(-5, 3),
            ],
            [
                Rational::new_from_args(2, -7),
                Rational::new_from_args(-9, -2),
                Rational::new_from_args(6, 5),
            ],
        ]
    );

    let m3 = Matrix::new(
        [
            [
                Rational::new_from_args(1, -2),
                Rational::new_from_args(0, 4),
                Rational::new_from_args(0, 3),
            ],
            [
                Rational::new_from_args(4, 1),
                Rational::new_from_args(0, -2),
                Rational::new_from_args(0, 5),
            ],
            [
                Rational::new_from_args(3, 1),
                Rational::new_from_args(0, -2),
                Rational::new_from_args(0, 5),
            ],
        ]
    );

    assert_eq!(m1.is_lower_triangular(), true);
    assert_eq!(m2.is_lower_triangular(), false);
    assert_eq!(m3.is_lower_triangular(), false);

}

#[test]
fn matrix_is_strictly_lower_triangular_test() {
    let m1 = Matrix::new(
        [
            [
                Rational::new_from_args(0, 2),
                Rational::new_from_args(3, 1),
                Rational::new_from_args(4, -2),
            ],
            [
                Rational::new_from_args(0, -2),
                Rational::new_from_args(0, 4),
                Rational::new_from_args(4, 3),
            ],
            [
                Rational::new_from_args(0, -2),
                Rational::new_from_args(0, 5),
                Rational::new_from_args(0, 1),
            ],
        ]
    );

    let m2 = Matrix::new(
        [
            [
                Rational::new_from_args(1, -2),
                Rational::new_from_args(3, 4),
                Rational::new_from_args(-5, 3),
            ],
            [
                Rational::new_from_args(2, -7),
                Rational::new_from_args(-9, -2),
                Rational::new_from_args(6, 5),
            ],
        ]
    );

    let m3 = Matrix::new(
        [
            [
                Rational::new_from_args(1, -2),
                Rational::new_from_args(0, 4),
                Rational::new_from_args(0, 3),
            ],
            [
                Rational::new_from_args(4, 1),
                Rational::new_from_args(0, -2),
                Rational::new_from_args(0, 5),
            ],
            [
                Rational::new_from_args(3, 1),
                Rational::new_from_args(0, -2),
                Rational::new_from_args(0, 5),
            ],
        ]
    );

    assert_eq!(m1.is_strictly_lower_triangular(), true);
    assert_eq!(m2.is_strictly_lower_triangular(), false);
    assert_eq!(m3.is_strictly_lower_triangular(), false);

}

#[test]
fn matrix_index_test() {
    let m = Matrix::new(
        [
            [
                Rational::new_from_args(1, -2),
                Rational::new_from_args(3, 4),
                Rational::new_from_args(-5, 3),
            ],
            [
                Rational::new_from_args(2, -7),
                Rational::new_from_args(-9, -2),
                Rational::new_from_args(6, 5),
            ],
        ]
    );

    assert_eq!(m[0][0], Rational::new_from_args(1, -2));
    assert_eq!(m[0][1], Rational::new_from_args(3, 4));
    assert_eq!(m[0][2], Rational::new_from_args(-5, 3));
    assert_eq!(m[1][0], Rational::new_from_args(2, -7));
    assert_eq!(m[1][1], Rational::new_from_args(-9, -2));
    assert_eq!(m[1][2], Rational::new_from_args(6, 5));
}

#[test]
fn matrix_negation_test() {
    let m = Matrix::new(
        [
            [
                Rational::new_from_args(1, -2),
                Rational::new_from_args(3, 4),
                Rational::new_from_args(-5, 3),
            ],
            [
                Rational::new_from_args(2, -7),
                Rational::new_from_args(-9, -2),
                Rational::new_from_args(6, 5),
            ],
        ]
    );

    assert_eq!(-m, Matrix::new(
        [
            [
                Rational::new_from_args(1, 2),
                Rational::new_from_args(-3, 4),
                Rational::new_from_args(5, 3),
            ],
            [
                Rational::new_from_args(2, 7),
                Rational::new_from_args(-9, 2),
                Rational::new_from_args(-6, 5),
            ],
        ]
    ));
}

#[test]
fn matrix_addition_test() {
    let m1 = Matrix::new(
        [
            [
                Rational::new_from_args(1, -2),
                Rational::new_from_args(3, 4),
                Rational::new_from_args(-5, 3),
            ],
            [
                Rational::new_from_args(2, -7),
                Rational::new_from_args(-9, -2),
                Rational::new_from_args(6, 5),
            ],
            [
                Rational::new_from_args(13, 3),
                Rational::new_from_args(2, -5),
                Rational::new_from_args(0, 5),
            ],
        ]
    );

    let m2 = Matrix::new(
        [
            [
                Rational::new_from_args(8, -3),
                Rational::new_from_args(3, 7),
                Rational::new_from_args(-5, 3),
            ],
            [
                Rational::new_from_args(3, -7),
                Rational::new_from_args(-9, 5),
                Rational::new_from_args(7, 57),
            ],
            [
                Rational::new_from_args(4, 1),
                Rational::new_from_args(0, -2),
                Rational::new_from_args(3, 5),
            ],
        ]
    );

    assert_eq!(m1 + m2, Matrix::new(
        [
            [
                Rational::new_from_args(-19, 6),
                Rational::new_from_args(33, 28),
                Rational::new_from_args(-10, 3),
            ],
            [
                Rational::new_from_args(-5, 7),
                Rational::new_from_args(27, 10),
                Rational::new_from_args(377, 285),
            ],
            [
                Rational::new_from_args(25, 3),
                Rational::new_from_args(2, -5),
                Rational::new_from_args(3, 5),
            ],
        ]
    ));
}


#[test]
fn matrix_subtraction_test() {
    let m1 = Matrix::new(
        [
            [
                Rational::new_from_args(1, -2),
                Rational::new_from_args(3, 4),
                Rational::new_from_args(-5, 3),
            ],
            [
                Rational::new_from_args(2, -7),
                Rational::new_from_args(-9, -2),
                Rational::new_from_args(6, 5),
            ],
            [
                Rational::new_from_args(13, 3),
                Rational::new_from_args(2, -5),
                Rational::new_from_args(0, 5),
            ],
        ]
    );

    let m2 = Matrix::new(
        [
            [
                Rational::new_from_args(8, -3),
                Rational::new_from_args(3, 7),
                Rational::new_from_args(-5, 3),
            ],
            [
                Rational::new_from_args(3, -7),
                Rational::new_from_args(-9, 5),
                Rational::new_from_args(7, 57),
            ],
            [
                Rational::new_from_args(4, 1),
                Rational::new_from_args(0, -2),
                Rational::new_from_args(3, 5),
            ],
        ]
    );

    assert_eq!(m1 - m2, Matrix::new(
        [
            [
                Rational::new_from_args(13, 6),
                Rational::new_from_args(9, 28),
                Rational::new_from_args(0, 1),
            ],
            [
                Rational::new_from_args(1, 7),
                Rational::new_from_args(63, 10),
                Rational::new_from_args(307, 285),
            ],
            [
                Rational::new_from_args(1, 3),
                Rational::new_from_args(2, -5),
                Rational::new_from_args(-3, 5),
            ],
        ]
    ));
}

#[test]
fn matrix_multiplication_test() {
    let m1 = Matrix::new(
        [
            [
                Rational::new_from_args(1, -2),
                Rational::new_from_args(3, 4),
                Rational::new_from_args(-5, 3),
            ],
            [
                Rational::new_from_args(2, -7),
                Rational::new_from_args(-9, -2),
                Rational::new_from_args(6, 5),
            ],
        ]
    );

    let m2 = Matrix::new(
        [
            [
                Rational::new_from_args(8, -3),
                Rational::new_from_args(3, 7),
            ],
            [
                Rational::new_from_args(3, -7),
                Rational::new_from_args(-9, 5),
            ],
            [
                Rational::new_from_args(4, 1),
                Rational::new_from_args(0, -2),
            ],
        ]
    );

    assert_eq!(m1 * m2, Matrix::new(
        [
            [
                Rational::new_from_args(178, 147),
                Rational::new_from_args(-1, 14),
                Rational::new_from_args(1562, 315),
            ],
            [
                Rational::new_from_args(51, 70),
                Rational::new_from_args(-1179, 140),
                Rational::new_from_args(-253, 175),
            ],
            [
                Rational::new_from_args(-2, 1),
                Rational::new_from_args(3, 1),
                Rational::new_from_args(-20, 3),
            ],
        ]
    ));
}

#[test]
fn matrix_scalar_multiplication_test() {
    let m1 = Matrix::new(
        [
            [
                Rational::new_from_args(1, -2),
                Rational::new_from_args(3, 4),
                Rational::new_from_args(-5, 3),
            ],
            [
                Rational::new_from_args(2, -7),
                Rational::new_from_args(-9, -2),
                Rational::new_from_args(6, 5),
            ],
        ]
    );

    assert_eq!(m1 * Rational::new_from_args(3, 4), Matrix::new(
        [
            [
                Rational::new_from_args(-3, 8),
                Rational::new_from_args(9, 16),
                Rational::new_from_args(-15, 12),
            ],
            [
                Rational::new_from_args(-3, 14),
                Rational::new_from_args(27, 8),
                Rational::new_from_args(9, 10),
            ],
        ]
    ));
}

#[test]
fn matrix_print_test() {
    let m = Matrix::new(
        [
            [
                Rational::new_from_args(1, -2),
                Rational::new_from_args(9, 28),
                Rational::new_from_args(-5, 3),
            ],
            [
                Rational::new_from_args(2, -7),
                Rational::new_from_args(2, -5),
                Rational::new_from_args(3, 5),
            ],
        ]
    );

    println!("{:?}", m);
}