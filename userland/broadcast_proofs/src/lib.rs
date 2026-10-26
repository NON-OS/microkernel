// NONOS Operating System (AGPL-3.0-or-later)
//! Host proofs for the broadcast deadline. `book` and `halves` are the
//! kernel's own files.

// The kernel's book has a const `new` for its static and no `Default`; the
// proofs give it one (book_default.rs).
#[path = "../../../src/arch/aarch64/timer/broadcast/book.rs"]
pub mod book;
mod book_default;
#[path = "../../../src/arch/aarch64/timer/broadcast/halves.rs"]
pub mod halves;

#[cfg(test)]
mod book_tests;
#[cfg(test)]
mod halves_tests;
#[cfg(test)]
mod handoff_tests;
#[cfg(test)]
mod model;
#[cfg(test)]
mod model_check;
#[cfg(test)]
mod model_tests;
#[cfg(test)]
mod order_tests;
