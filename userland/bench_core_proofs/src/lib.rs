// NONOS Operating System (AGPL-3.0-or-later)
//! Host proofs for the benchmark summary maths.
//!
//! A percentile that is quietly off by one reports a number nobody can
//! reproduce, and the entire purpose of publishing these figures is that
//! someone else can. So the reduction is driven here against runs whose answers
//! are known by hand.
//!
//! The cycle counter itself is not proved here and cannot be: on the host
//! `read_serialised` returns a constant by construction, which is why it does
//! so loudly rather than returning a plausible number.

pub mod core;

#[cfg(test)]
mod summary_tests;
