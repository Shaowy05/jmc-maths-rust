use super::field::Field;
use super::vector::Vector;
use std::fmt::{self, Debug, Formatter};
use std::ops::Index;

pub struct Matrix<T: Field, const N: usize, const M: usize>([[T; N]; M]);

impl<T: Field, const N: usize, const M: usize> Matrix<T, N, M> {
    pub fn new(elements: [[T; N]; M]) -> Self {
        Matrix(elements)
    }

    pub fn new_from_column_vectors(vectors: [Vector<T, N>; M]) -> Self {
        let mut elements = [[T::additive_identity(); N]; M];

        for i in 0..M {
            for j in 0..N {
                elements[i][j] = vectors[i][j];
            }
        }

        Matrix::new(elements)
    }

    pub fn new_from_row_vectors(vectors: [Vector<T, M>; N]) -> Self {
        let mut elements = [[T::additive_identity(); N]; M];

        for i in 0..M {
            for j in 0..N {
                elements[i][j] = vectors[j][i];
            }
        }

        Matrix::new(elements)
    }

    pub fn size(&self) -> (usize, usize) {
        (N, M)
    }

    pub fn number_of_rows(&self) -> usize {
        N
    }

    pub fn number_of_columns(&self) -> usize {
        M
    }

    pub fn identity() -> Self {
        let mut elements = [[T::additive_identity(); N]; M];

        for i in 0..N {
            for j in 0..M {
                if i == j {
                    elements[i][j] = T::multiplicative_identity();
                }
            }
        }

        Matrix::new(elements)
    }
}

impl<T: Field, const N: usize, const M: usize> Index<usize> for Matrix<T, N, M> {
    type Output = [T; N];

    fn index(&self, index: usize) -> &[T; N] {
        &self.0[index]
    }
}

impl<T: Field, const N: usize, const M: usize> Debug for Matrix<T, N, M> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Matrix{{")?;

        for i in 0..M {
            write!(f, "[")?;

            for j in 0..N {
                if j == N - 1 {
                    write!(f, "{:?}", self.0[i][j])?;
                } else {
                    write!(f, "{:?}, ", self.0[i][j])?;
                }
            }

            if i == M - 1 {
                write!(f, "]")?;
            } else {
                write!(f, "], ")?;
            }
        }

        write!(f, "}}")
    }
}