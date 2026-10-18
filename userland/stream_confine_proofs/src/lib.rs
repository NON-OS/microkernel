// NONOS Operating System (AGPL-3.0-or-later)
//! Host proofs for confining described devices. `verdict` and the
//! described `iommus` type are the kernel's own files.

#![allow(dead_code)]

#[path = "../../../src/hardware/broker/described/iommus.rs"]
pub mod described_iommus;
#[path = "../../../src/hardware/broker/confine/verdict.rs"]
pub mod verdict;

pub mod hardware {
    pub mod broker {
        pub mod described {
            pub use crate::described_iommus::{DescribedIommus, MAX_DESCRIBED_STREAMS};
        }
    }
}

#[cfg(test)]
mod order_tests;
#[cfg(test)]
mod verdict_tests;
