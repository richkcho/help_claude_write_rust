//! Error handling patterns in Rust
//!
//! Shows various approaches to error handling that LLMs often struggle with

use std::fmt;
use std::error::Error;
use std::io;

/// Custom error type implementation
/// This is a common pattern for library crates
#[derive(Debug)]
pub enum ParseError {
    InvalidFormat,
    OutOfRange,
    IoError(io::Error),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::InvalidFormat => write!(f, "Invalid format"),
            ParseError::OutOfRange => write!(f, "Value out of range"),
            ParseError::IoError(e) => write!(f, "IO error: {}", e),
        }
    }
}

impl Error for ParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ParseError::IoError(e) => Some(e),
            _ => None,
        }
    }
}

impl From<io::Error> for ParseError {
    fn from(err: io::Error) -> Self {
        ParseError::IoError(err)
    }
}

/// Using Result<T, E> - the standard Rust pattern
pub fn parse_number(s: &str) -> Result<i32, ParseError> {
    if s.is_empty() {
        return Err(ParseError::InvalidFormat);
    }
    
    s.parse::<i32>()
        .map_err(|_| ParseError::InvalidFormat)
}

/// Chaining results with ? operator
pub fn process_numbers(input1: &str, input2: &str) -> Result<i32, ParseError> {
    let num1 = parse_number(input1)?;
    let num2 = parse_number(input2)?;
    Ok(num1 + num2)
}

/// Result with multiple error types using Box<dyn Error>
pub fn flexible_error() -> Result<String, Box<dyn Error>> {
    let num = "42".parse::<i32>()?;
    Ok(format!("Parsed: {}", num))
}

/// Using Option for values that might not exist
pub fn safe_divide(x: i32, y: i32) -> Option<i32> {
    if y == 0 {
        None
    } else {
        Some(x / y)
    }
}

/// Combining Option and Result
pub fn parse_and_divide(s: &str, divisor: i32) -> Result<Option<i32>, ParseError> {
    let num = parse_number(s)?;
    Ok(safe_divide(num, divisor))
}

/// Using unwrap_or and unwrap_or_else for defaults
pub fn get_value_or_default(opt: Option<i32>) -> i32 {
    opt.unwrap_or(42)
}

/// Pattern matching on Result
pub fn handle_result(r: Result<i32, ParseError>) -> String {
    match r {
        Ok(val) => format!("Success: {}", val),
        Err(ParseError::InvalidFormat) => "Invalid format".to_string(),
        Err(ParseError::OutOfRange) => "Out of range".to_string(),
        Err(ParseError::IoError(e)) => format!("IO error: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_number() {
        assert!(parse_number("42").is_ok());
        assert!(parse_number("").is_err());
        assert!(parse_number("abc").is_err());
    }
    
    #[test]
    fn test_safe_divide() {
        assert_eq!(safe_divide(10, 2), Some(5));
        assert_eq!(safe_divide(10, 0), None);
    }
}
