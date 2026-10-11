// NONOS Operating System (AGPL-3.0-or-later)
//! Host proofs for the aarch64 console UART's pure part. `uart` mirrors the
//! kernel's module layout so the mounted file sits where it does there.

pub mod uart;

#[cfg(test)]
mod divisor_tests;
