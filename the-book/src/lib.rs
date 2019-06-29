//! # My Rust Cheatsheet
//! This documentation is general for the crate.
//!

pub use nested1::nested2::PrimaryColor;

// Use 3 slashes to write documentation for the api.
// To generate it, use:
//    cargo doc -- open
// The tests in the documentation will run! (seems it doesn't work in main.rs, but works in lib.rs)
// Example documentation:

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = the_book::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

// Just to show pub use
pub mod nested1 {
    pub mod nested2 {
        pub enum PrimaryColor {
            Red,
            Yellow,
            Blue,
        }
    }
}
