use super::field::Field;
use std::ops::{Add, Index, Mul, Sub};

#[derive(Debug, PartialEq, Clone)]
pub struct Vector<T: Field> {
    elements: Vec<T>,
}

impl <T: Field> Vector<T>{
    fn new(elements: Vec<T>) -> Vector<T> {
        Vector { elements }
    }

    fn size(&self) -> usize {
        self.elements.len()
    }
}

impl <T: Field> Index<usize> for Vector<T> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        &self.elements[index]
    }
}

impl<T: Field> Add for Vector<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        if self.size() != other.size() {
            panic!("Vectors must have the same size");
        }
        let elements = self.elements.iter()
            .zip(other.elements.iter())
            .map(|(a, b)| a.clone() + b.clone())
            .collect();

        Vector::new(elements)
    }
    
}

impl <T: Field> Sub for Vector<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        if self.size() != other.size() {
            panic!("Vectors must have the same size");
        }
        let elements = self.elements.iter()
            .zip(other.elements.iter())
            .map(|(a, b)| a.clone() - b.clone())
            .collect();

        Vector::new(elements)
    }
    
}

impl <T: Field> Mul<T> for Vector<T> {
    type Output = Vector<T>;

    fn mul(self, other: T) -> Vector<T> {
        let elements = self.elements.iter()
            .map(|e| e.clone() * other)
            .collect();

        Vector::new(elements)
    }
    
}

impl <T: Field> Mul<Vector<T>> for Vector<T> {
    type Output = T;

    fn mul(self, other: Vector<T>) -> T {
        if self.size() != other.size() {
            panic!("Vectors must have the same size");
        }
        let elements = self.elements.iter()
            .zip(other.elements.iter())
            .map(|(a, b)| a.clone() * b.clone())
            .collect::<Vec<T>>();

        elements.iter().fold(T::additive_identity(), |acc, e| acc + *e)
    }
    
}