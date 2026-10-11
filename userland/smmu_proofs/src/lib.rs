// NONOS Operating System (AGPL-3.0-or-later)
//! Host proofs for the SMMUv3 driver's pure parts. `smmu` mirrors the
//! kernel's module layout so the mounted files sit where they do there.

pub mod smmu;

#[cfg(test)]
mod encode_tests;
#[cfg(test)]
mod idr_tests;
#[cfg(test)]
mod queue_tests;
