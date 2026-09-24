// NONOS Operating System (AGPL-3.0-or-later)
//! The crate's own module shape, so the shipping files compile unchanged.
//! `overhead` reaches for `tsc` and `summary` as siblings exactly as it does in
//! `bench_core`, so the tree has to match.

#[allow(dead_code)]
#[path = "../../../../nonos-bench/src/tsc.rs"]
pub mod tsc;
#[allow(dead_code)]
#[path = "../../../../nonos-bench/src/sort.rs"]
pub mod sort;
#[allow(dead_code)]
#[path = "../../../../nonos-bench/src/summary.rs"]
pub mod summary;
#[allow(dead_code)]
#[path = "../../../../nonos-bench/src/overhead.rs"]
pub mod overhead;
