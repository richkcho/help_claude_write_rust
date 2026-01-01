//! Examples of lifetime annotations and borrowing patterns
//!
//! Lifetimes are one of the most challenging aspects of Rust for LLMs.
//! These examples show common patterns.

/// Returns the longer of two string slices
/// This demonstrates basic lifetime annotations
pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

/// A struct that holds a reference
/// This shows how to annotate struct fields with lifetimes
pub struct Excerpt<'a> {
    pub part: &'a str,
}

impl<'a> Excerpt<'a> {
    /// Returns the part with a prefix
    pub fn announce(&self, prefix: &str) -> String {
        format!("{}: {}", prefix, self.part)
    }
}

/// Multiple lifetime parameters
/// When you need to track different borrowed values
pub fn split_and_first<'a, 'b>(first: &'a str, second: &'b str) -> (&'a str, &'b str) {
    (first, second)
}

/// Demonstrates lifetime elision rules
/// The compiler can infer these lifetimes
pub fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    s
}

/// Static lifetime - data that lives for the entire program
pub fn get_static_str() -> &'static str {
    "This string lives forever"
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_longest() {
        let s1 = "hello";
        let s2 = "world!";
        assert_eq!(longest(s1, s2), "world!");
    }
    
    #[test]
    fn test_excerpt() {
        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.').next().unwrap();
        let excerpt = Excerpt { part: first_sentence };
        assert_eq!(excerpt.announce("Book"), "Book: Call me Ishmael");
    }
}
