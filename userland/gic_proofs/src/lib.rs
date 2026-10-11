// NONOS Operating System (AGPL-3.0-or-later)
//! Host proofs for the aarch64 GIC's pure parts. `gic` mirrors the kernel's
//! module layout so the `super::` paths inside the mounted files resolve.

pub mod gic;

#[cfg(test)]
mod encode_tests;
#[cfg(test)]
mod intid_tests;
#[cfg(test)]
mod select_tests;
