use super::field::Field;
use super::vector::Vector;
use std::fmt::{self, Debug, Formatter};
use std::ops::Index;

#[derive(Clone, PartialEq)]
pub struct Matrix<T: Field, const M: usize, const N: usize>([[T; M]; N]);

impl<T: Field, const M: usize, const N: usize> Matrix<T, M, N> {
    pub fn new(elements: [[T; M]; N]) -> Self {
        Matrix(elements)
    }

    pub fn new_from_column_vectors(vectors: [Vector<T, M>; N]) -> Self {
        let mut elements = [[T::additive_identity(); M]; N];

        for i in 0..N {
            elements[i] = vectors[i].as_array();
        }

        Matrix::new(elements)
    }

    pub fn new_from_row_vectors(vectors: [Vector<T, N>; M]) -> Self {
        let mut elements = [[T::additive_identity(); M]; N];

        for i in 0..M {
            for j in 0..N {
                elements[i][j] = vectors[i][j];
            }
        }

        Matrix::new(elements)
    }

    pub fn size(&self) -> (usize, usize) {
        (M, N)
    }

    pub fn number_of_rows(&self) -> usize {
        M
    }

    pub fn number_of_columns(&self) -> usize {
        N
    }

    pub fn identity() -> Matrix<T, M, M> {
        let mut elements = [[T::additive_identity(); M]; M];

        for i in 0..N {
            elements[i][i] = T::multiplicative_identity();
        }

        Matrix::new(elements)
    }

    pub fn transpose(&self) -> Matrix<T, N, M> {
        let mut elements = [[T::additive_identity(); N]; M];

        for i in 0..N {
            for j in 0..M {
                elements[j][i] = self.0[i][j];
            }
        }

        Matrix::new(elements)
    }

    pub fn column(&self, index: usize) -> Vector<T, M> {
        let elements = self.0[index];

        Vector::new(elements)
    }

    pub fn row(&self, index: usize) -> Vector<T, N> {
        let mut elements = [T::additive_identity(); N];

        for i in 0..N {
            elements[i] = self.0[i][index];
        }

        Vector::new(elements)
    }

    pub fn is_square(&self) -> bool {
        N == M
    }

    pub fn is_upper_triangular(&self) -> bool {
        if N != M {
            return false;
        }

        for i in 0..N-1 {
            for j in (i + 1..M).rev(){
                if self.0[i][j] != T::additive_identity() {
                    return false;
                }
            }
        }

        true
    }

    pub fn is_strictly_upper_triangular(&self) -> bool {
        if N != M {
            return false;
        }

        for i in 0..N {
            for j in (i..M).rev() {
                if self.0[i][j] != T::additive_identity() {
                    return false;
                }
            }
        }

        true
    }

    pub fn is_lower_triangular(&self) -> bool {
        if N != M {
            return false;
        }

        for i in 1..N {
            for j in 0..i {
                if self.0[i][j] != T::additive_identity() {
                    return false;
                }
            }
        }

        true
    }

    pub fn is_strictly_lower_triangular(&self) -> bool {
        if N != M {
            return false;
        }

        for i in 0..N {
            for j in 0..i {
                if self.0[i][j] != T::additive_identity() {
                    return false;
                }
            }
        }

        true
    }
}

impl<T: Field, const N: usize, const M: usize> Index<usize> for Matrix<T, M, N> {
    type Output = [T; M];

    fn index(&self, index: usize) -> &[T; M] {
        &self.0[index]
    }
}

impl<T: Field, const M: usize, const N: usize> Debug for Matrix<T, M, N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Matrix{{")?;

        for i in 0..N {
            write!(f, "{:?}", self.0[i])?;
        }

        write!(f, "}}")
    }
}
