//! The v2 circuit's pinned shape A, which a launch proof is judged against.

use super::super::report::Shape;

/// Pinned from the v2 package, never read from the proof being judged: shape A's parameter id.
pub(super) const V2_PARAMS_ID: &str =
    "ccba76ed5748b5ee54dfd62935fd1d10998b5c0f84e35a9dc899905cfb1eadb5";

/// The v2 circuit at shape A, from the package's `PARAMS.md`.
pub(super) const LAUNCH_SHAPE: Shape = Shape {
    log_trace_len: 13,
    trace_width: 44,
    constraint_degree: 11,
    periodic_columns: 59,
    public_words: 36,
    queries: 19,
    grind_bits: 28,
    extra_blowup_bits: 5,
    security_bits: 80,
};
