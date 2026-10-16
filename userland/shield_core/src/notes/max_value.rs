//! The largest value a note may hold.

/// The largest value a note may hold, `p - 2`, the pool's `MAX_VALUE`. The
/// circuit adds values in the field, so a larger one could wrap.
pub const MAX_VALUE: u64 = nonos_stark::field::P - 2;
