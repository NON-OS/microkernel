//! The card holds the run's numbers and says so where it matters.
//!
//! This is the artifact a film cites and a later run is diffed against, so
//! what it must not do is quietly lose a line or print a failed verification
//! as though it passed.

/*
 * A test asserts by panicking, and this one builds a report by hand, so the
 * lints that forbid panicking and indexing are off here and nowhere else.
 */
#![allow(
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::panic
)]

use super::report::Shape;
use super::{card, BenchReport};

fn report(verified: bool) -> BenchReport {
    BenchReport {
        built_ms: 17,
        proved_ms: 74_292,
        verified_ms: 236,
        verified,
        proof_bytes: 1_017_516,
        peak_kib: Some(228_864),
        shape: Shape {
            log_trace_len: 13,
            trace_width: 20,
            constraint_degree: 11,
            periodic_columns: 70,
            public_words: 29,
            queries: 64,
            grind_bits: 16,
            extra_blowup_bits: 1,
            security_bits: 144,
        },
        params_id: String::new(),
        launch_point: false,
        proof_path: None,
    }
}

/// Every number the run produced is in the card, and so is the shape that says
/// which instance produced them.
#[test]
fn the_card_carries_the_run() {
    let text = card(&report(true), "x86_64, iOS 18.6");
    for line in [
        "proved: 74.2 s (74292 ms)",
        "verified: 236 ms, true",
        "proof: 1017516 bytes",
        "soundness: 144 bits, 64 queries at grind 16",
        "peak: 228864 KiB",
        "rows: 2^13",
        "columns: 20",
        "where: x86_64, iOS 18.6",
    ] {
        assert!(text.contains(line), "the card is missing {line}, it reads:\n{text}");
    }
}

/// A proof that did not verify voids the timing, so the card says so in a word
/// nobody reads as success.
#[test]
fn a_failed_verification_is_not_printed_as_true() {
    let text = card(&report(false), "somewhere");
    assert!(text.contains("verified: 236 ms, FALSE"), "the card reads:\n{text}");
    assert!(!text.contains(", true"), "the card claims a verification it did not get");
}
