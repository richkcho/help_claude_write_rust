//! Generic type examples
//!
//! Generics allow writing flexible, reusable code. These patterns are
//! particularly challenging for LLMs to get right.

use std::fmt::Display;

/// Simple generic function
pub fn first_element<T>(list: &[T]) -> Option<&T> {
    list.first()
}

/// Generic function with trait bounds
pub fn print_value<T: Display>(value: T) {
    println!("{}", value);
}

/// Multiple generic type parameters
pub fn make_pair<T, U>(first: T, second: U) -> (T, U) {
    (first, second)
}

/// Generic struct
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
}

/// Implementing methods only for specific types
impl Point<f64> {
    pub fn distance_from_origin(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

/// Generic struct with multiple type parameters
pub struct Pair<T, U> {
    pub first: T,
    pub second: U,
}

impl<T, U> Pair<T, U> {
    pub fn new(first: T, second: U) -> Self {
        Pair { first, second }
    }
    
    pub fn swap(self) -> Pair<U, T> {
        Pair {
            first: self.second,
            second: self.first,
        }
    }
}

/// Generic with where clause for complex bounds
pub fn complex_bound<T, U>(t: T, u: U) -> String
where
    T: Display + Clone,
    U: Display + PartialEq,
{
    format!("{} and {}", t, u)
}

/// Generic enum - like Option and Result
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

impl<L, R> Either<L, R> {
    pub fn is_left(&self) -> bool {
        matches!(self, Either::Left(_))
    }
    
    pub fn is_right(&self) -> bool {
        matches!(self, Either::Right(_))
    }
}

/// Using generic bounds in struct definitions
pub struct Wrapper<T: Display> {
    pub value: T,
}

impl<T: Display> Wrapper<T> {
    pub fn new(value: T) -> Self {
        Wrapper { value }
    }
    
    pub fn display(&self) -> String {
        format!("Wrapped: {}", self.value)
    }
}

/// Lifetime and generic parameters together
pub struct RefPair<'a, T> {
    pub first: &'a T,
    pub second: &'a T,
}

impl<'a, T> RefPair<'a, T> {
    pub fn new(first: &'a T, second: &'a T) -> Self {
        RefPair { first, second }
    }
}

/// Generic function with lifetime bounds
pub fn longest_string<'a, T>(x: &'a T, y: &'a T) -> &'a T
where
    T: AsRef<str>,
{
    if x.as_ref().len() > y.as_ref().len() {
        x
    } else {
        y
    }
}

/// Const generics - arrays with generic size
pub struct FixedBuffer<T, const N: usize> {
    pub data: [T; N],
}

impl<T: Default + Copy, const N: usize> FixedBuffer<T, N> {
    pub fn new() -> Self {
        FixedBuffer {
            data: [T::default(); N],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_point() {
        let p = Point::new(3.0, 4.0);
        assert_eq!(p.distance_from_origin(), 5.0);
    }
    
    #[test]
    fn test_pair() {
        let p = Pair::new("hello", 42);
        let swapped = p.swap();
        assert_eq!(swapped.first, 42);
    }
    
    #[test]
    fn test_either() {
        let left: Either<i32, String> = Either::Left(42);
        assert!(left.is_left());
    }
}
