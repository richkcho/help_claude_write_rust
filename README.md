# help_claude_write_rust

A collection of Rust code patterns that help LLM agents understand common challenges.

## Purpose

This crate contains compilable examples of Rust patterns that AI assistants (like Claude) often struggle with. Each module demonstrates specific patterns with working code that can be referenced when generating or reviewing Rust code.

## Modules

### `lifetimes`
Examples of lifetime annotations and borrowing patterns:
- Basic lifetime annotations
- Structs with lifetime parameters
- Multiple lifetime parameters
- Lifetime elision rules
- Static lifetimes

### `error_handling`
Various approaches to error handling:
- Custom error types
- Result and Option types
- Error propagation with `?` operator
- Combining different error types
- Pattern matching on errors

### `traits`
Trait definitions and implementations:
- Basic trait definition and implementation
- Traits with default implementations
- Trait bounds and where clauses
- Trait objects and dynamic dispatch
- Associated types
- Blanket implementations
- Trait inheritance (supertraits)

### `generics`
Generic type examples:
- Generic functions and structs
- Multiple type parameters
- Generic bounds with where clauses
- Generic enums
- Lifetime and generic parameters together
- Const generics

### `async_patterns`
Asynchronous programming patterns:
- Basic async/await
- Async functions with error handling
- Async trait methods
- Boxed futures
- Async with lifetime parameters
- Async iterators (stream-like patterns)

### `borrowing_patterns`
Advanced borrowing challenges and solutions:
- Double mutable borrow from same vector (multiple solutions)
- 2D matrix with multiple mutable borrows (same row, different rows)
- Enum-indexed data structures with multiple mutable access
- Safe `split_at_mut` patterns
- Generic helper functions for getting multiple mutable references

## Usage

This is a library crate intended as a reference. Build and test with:

```bash
cargo build
cargo test
```

## License

This project is provided as educational reference material.
