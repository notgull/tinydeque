// MIT/Apache2 License

#![no_std]
#![warn(clippy::pedantic)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::redundant_pattern_matching)]
#![allow(clippy::cast_possible_wrap)]
#![forbid(unsafe_code)]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod array_deque;
#[cfg(feature = "alloc")]
pub mod tiny_deque;

pub use array_deque::ArrayDeque;
#[cfg(feature = "alloc")]
pub use tiny_deque::TinyDeque;
