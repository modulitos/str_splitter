# String Splitter

An exercise from this workshop by Jon Gjengset:
https://www.youtube.com/watch?v=rAl-9HwD858

Where we implement the ability to split a string slice according to some delimeter.

We are basically implementing a simplified version of the `Split` struct from the standard library:
https://doc.rust-lang.org/alloc/str/struct.Split.html


## This create is broken down into parts:
Each part builds upon the part before it.

Part 1: Basic implementation, using a single lifetime for both the str and the delimeter
Part 2: Handle a tricky edge case
Part 3: Refactor to separate the lifetime coupling by using multiple lifetimes
Part 4: Refactor to remove the second lifetime by re-implementing the delimeter as a trait

## Lessons learned in each part:

This implementation is separated into 4 phases:
Part 1:
 * basic lifetime support when returning `&str` references from a function
Part 2:
 * mutable references to &str
 * having to use `as_mut` on an `Option<&str>`, because `&str` implements `Copy`.
Part 3:
 * Lifetime elision (aka "anonymous lifetimes")
 * Working with multiple lifetimes, and identifying the situations where they are needed (usually, you do not need multiple lifetimes)
Part 4:
 * Converting the second lifetime into a generic type D, and having D implement a Delimeter trait where needed

