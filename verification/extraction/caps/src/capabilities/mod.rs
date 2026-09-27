// NONOS Operating System (AGPL-3.0-or-later)
// The real capability enum and bit-token operations from src/capabilities, plus
// the decision cores the kernel factored out of their structures so they could
// be checked: the delegation expiry meet, the quota comparison, the nonce
// composition and the chain depth bound. Each of those files says in its own
// header that it exists to be verified; these are the extractions.
#[path = "../../../../../src/capabilities/types/mod.rs"]
pub mod types;

#[path = "../../../../../src/capabilities/bits.rs"]
pub mod bits;

pub mod chain;
pub mod delegation;
pub mod resource;
