// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/sys/microbench/sample.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/sys/microbench/sample.rs"]
pub mod sample;

pub fn sample_new() -> sample::Sample {
    sample::Sample::new()
}

pub fn sample_len(this: sample::Sample) -> usize {
    this.len()
}

pub fn sample_quantile(this: sample::Sample, numerator: usize, denominator: usize) -> u64 {
    this.quantile(numerator, denominator)
}

pub fn sample_min(this: sample::Sample) -> u64 {
    this.min()
}

pub fn sample_max(this: sample::Sample) -> u64 {
    this.max()
}

