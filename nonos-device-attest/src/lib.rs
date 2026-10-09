// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! The anonymous device statement.
//!
//! A device proves, to a verifier that learns nothing else, that
//!
//! - an approved bootloader started it: a slot of kind `Bootloader` in B;
//! - an approved kernel runs on it: a slot of kind `Kernel` in P;
//! - it is an enrolled device: `commit(s)` is a leaf of R;
//! - and `t` is its tag for the verifier's scope `e`, bound to the context `c`.
//!
//! Which bootloader, which kernel, which device and `s` stay private. One device
//! has one tag per scope, so a verifier can rate-limit without identifying.
//!
//! The proof shows that an approved chain exists and that this device holds an
//! enrolled secret. That the approved chain is the one running is the sealing's
//! guarantee: the TPM derives `s` only under a PCR 9 value the release signed
//! for (PolicyAuthorize) and under PCRs 0, 4 and 7 as this machine found them.

#![cfg_attr(not(test), no_std)]

extern crate alloc;

mod circuit;
mod domain;
mod native;
mod params;
mod prove;
mod registry;
mod statement;
mod witness;

#[cfg(test)]
mod tests;

pub use domain::{DEVICE_DOMAIN, TAG_DOMAIN};
pub use native::{commit, leaf, scope, tag, words_of};
pub use params::{BOOT_DEPTH, KERNEL_DEPTH, SHIPPED_DEVICE_DEPTH};
pub use prove::{prove, verify, Error, Proof};
pub use registry::{ek_id, Enrolled, Registry, RegistryError, MAX_DEVICE_DEPTH};
pub use statement::Statement;
pub use witness::{Path, Slot, Witness};
