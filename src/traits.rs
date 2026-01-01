//! Trait definitions and implementations
//!
//! Traits are Rust's way of defining shared behavior. These examples
//! show common patterns that LLMs often need help with.

use std::fmt;

/// Basic trait definition
pub trait Drawable {
    fn draw(&self) -> String;
}

/// Struct implementing a trait
pub struct Circle {
    pub radius: f64,
}

impl Drawable for Circle {
    fn draw(&self) -> String {
        format!("Circle with radius {}", self.radius)
    }
}

pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

impl Drawable for Rectangle {
    fn draw(&self) -> String {
        format!("Rectangle {}x{}", self.width, self.height)
    }
}

/// Trait with default implementation
pub trait Describable {
    fn name(&self) -> &str;

    fn description(&self) -> String {
        format!("This is a {}", self.name())
    }
}

impl Describable for Circle {
    fn name(&self) -> &str {
        "circle"
    }
}

/// Trait bounds in function signatures
pub fn draw_twice<T: Drawable>(item: &T) -> String {
    format!("{}\n{}", item.draw(), item.draw())
}

/// Multiple trait bounds
pub fn draw_and_describe<T>(item: &T) -> String
where
    T: Drawable + Describable,
{
    format!("{}: {}", item.description(), item.draw())
}

/// Trait objects for dynamic dispatch
pub fn draw_all(items: &[Box<dyn Drawable>]) -> Vec<String> {
    items.iter().map(|item| item.draw()).collect()
}

/// Associated types in traits
pub trait Container {
    type Item;

    fn add(&mut self, item: Self::Item);
    fn get(&self, index: usize) -> Option<&Self::Item>;
}

pub struct NumberContainer {
    items: Vec<i32>,
}

impl Container for NumberContainer {
    type Item = i32;

    fn add(&mut self, item: i32) {
        self.items.push(item);
    }

    fn get(&self, index: usize) -> Option<&i32> {
        self.items.get(index)
    }
}

/// Blanket implementation - implementing a trait for any type that satisfies bounds
pub trait Printable {
    fn print(&self);
}

impl<T: fmt::Display> Printable for T {
    fn print(&self) {
        println!("{}", self);
    }
}

/// Trait inheritance (supertraits)
pub trait Shape: Drawable {
    fn area(&self) -> f64;
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

/// Marker traits (zero-sized traits)
pub trait Serializable {}

impl Serializable for Circle {}
impl Serializable for Rectangle {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drawable() {
        let circle = Circle { radius: 5.0 };
        assert!(circle.draw().contains("Circle"));
    }

    #[test]
    fn test_trait_objects() {
        let shapes: Vec<Box<dyn Drawable>> = vec![
            Box::new(Circle { radius: 5.0 }),
            Box::new(Rectangle {
                width: 10.0,
                height: 20.0,
            }),
        ];
        let drawings = draw_all(&shapes);
        assert_eq!(drawings.len(), 2);
    }
}
