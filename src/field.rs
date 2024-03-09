use std::{fmt::Debug, ops::{Add, Div, Mul, Neg, Sub}};

pub trait Field:
    Add<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Mul<Self, Output = Self>
    + Div<Self, Output = Self>
    + Neg<Output = Self>
    + PartialEq
    + Clone
    + Copy
    + Debug
{
    fn additive_identity() -> Self;
    fn multiplicative_identity() -> Self;
    fn additive_inverse(&self) -> Self;
    fn multiplicative_inverse(&self) -> Self;
}
