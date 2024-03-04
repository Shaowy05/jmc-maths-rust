use super::field::Field;
use std::ops::{Add, Index, Mul, Sub};

pub struct Vector<T: Field, const N: usize>([T; N]);

impl <T: Field, const N: usize> Vector<T, N> {
    pub fn new(elements: [T; N]) -> Vector<T, N> {
        Vector(elements)
    }

    pub fn size() -> usize {
        N
    }
}

impl<T: Field, const N: usize> Index<usize> for Vector<T, N> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        &self.0[index]
    }
}

impl<T: Field, const N: usize> Add for Vector<T, N> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut new_elements = [T::additive_identity(); N];

        for i in 0..N {
            new_elements[i] = self.0[i] + other.0[i];
        }

        Vector::new(new_elements)
    }
}

impl<T: Field, const N: usize> Sub for Vector<T, N> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let mut new_elements = [T::additive_identity(); N];

        for i in 0..N {
            new_elements[i] = self.0[i] - other.0[i];
        }

        Vector::new(new_elements)
    }
}

impl<T: Field, const N: usize> Mul<T> for Vector<T, N> {
    type Output = Vector<T, N>;

    fn mul(self, other: T) -> Vector<T, N> {
        let mut new_elements = [T::multiplicative_identity(); N];

        for i in 0..N {
            new_elements[i] = self.0[i] * other;
        }

        Vector::new(new_elements)
    }
}

impl<T: Field, const N: usize> Mul<Vector<T, N>> for Vector<T, N> {
    type Output = T;

    fn mul(self, other: Vector<T, N>) -> T {
        let mut result = T::additive_identity();

        for i in 0..N {
            result = result + (self.0[i] * other.0[i]);
        }

        result
    }
}