//! The verified kernel.
//!
//! Every function here is written so Charon can translate it and Aeneas can turn it into
//! Lean: explicit arithmetic, no closures, no iterators, no allocation, no panics. The
//! proofs in `lean/` are about this code, not a hand written model of it.

//! Nothing in here knows what a wallet is. It is arithmetic and layout, and the core
//! calls it.
//!
//! The proofs are the reason the crate exists.

#![no_std]

pub mod amount;
pub mod base32;
pub mod base32_tail;
pub mod fee;
pub mod limbs;
pub mod transfer;
