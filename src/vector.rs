use super::field::Field;
use std::{fmt::Debug, ops::{Add, Index, Mul, Sub}};

#[derive(Clone)]
pub struct Vector<T: Field, const M: usize>([T; M]);

impl <T: Field, const M: usize> Vector<T, M> {
    pub fn new(elements: [T; M]) -> Vector<T, M> {
        Vector(elements)
    }

    pub fn dimension() -> usize {
        M
    }

    pub fn size(&self) -> usize {
        M
    }

    pub fn as_array(&self) -> [T; M] {
        self.0.clone()
    }
}

impl<T: Field, const M: usize> Index<usize> for Vector<T, M> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        &self.0[index]
    }
}

impl<T: Field, const M: usize> Add for Vector<T, M> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut elements = [T::additive_identity(); M];

        for i in 0..M {
            elements[i] = self.0[i] + other.0[i];
        }

        Vector::new(elements)
    }
}

impl<T: Field, const M: usize> Sub for Vector<T, M> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let mut elements = [T::additive_identity(); M];

        for i in 0..M {
            elements[i] = self.0[i] - other.0[i];
        }

        Vector::new(elements)
    }
}

impl<T: Field, const M: usize> Mul<T> for Vector<T, M> {
    type Output = Vector<T, M>;

    fn mul(self, other: T) -> Vector<T, M> {
        let mut elements = [T::multiplicative_identity(); M];

        for i in 0..M {
            elements[i] = self.0[i] * other;
        }

        Vector::new(elements)
    }
}

impl<T: Field, const M: usize> Mul<Vector<T, M>> for Vector<T, M> {
    type Output = T;

    fn mul(self, other: Vector<T, M>) -> T {
        let mut result = T::additive_identity();

        for i in 0..M {
            result = result + (self.0[i] * other.0[i]);
        }

        result
    }
}

impl<T: Field, const M: usize> PartialEq for Vector<T, M> {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..M {
            if self.0[i] != other.0[i] {
                return false;
            }
        }

        true
    }
}

impl<T: Field, const M: usize> Debug for Vector<T, M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<T: Field, const M: usize> Copy for Vector<T, M> {}