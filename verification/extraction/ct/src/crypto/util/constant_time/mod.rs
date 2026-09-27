// NONOS Operating System (AGPL-3.0-or-later)
// The comparison primitives from src/crypto/util/constant_time, and the
// barriers module they read their fences from. The re-export mirrors the real
// module's, because compare.rs reaches for the fences through `super`.
#[path = "../../../../../../../src/crypto/util/constant_time/barriers.rs"]
pub mod barriers;

#[path = "../../../../../../../src/crypto/util/constant_time/compare.rs"]
pub mod compare;

pub use barriers::{compiler_fence, volatile_read};
