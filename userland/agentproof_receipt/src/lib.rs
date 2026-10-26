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
//! AgentProof receipt v0: the NAPR format (`napr`), what a verifier trusts
//! (`trust`), and the offline check (`verify`). Nothing a receipt carries is
//! taken on its word: the box, the app and the doc's challenge are each
//! recomputed from its bytes.

#![no_std]

extern crate alloc;

#[cfg(feature = "verify")]
pub mod cli;
#[cfg(feature = "verify")]
pub mod cli_pin;
#[cfg(feature = "verify")]
pub mod cli_ran;
#[cfg(feature = "verify")]
pub mod cli_says;
pub mod napr;
#[cfg(feature = "verify")]
pub mod trust;
#[cfg(feature = "verify")]
pub mod trust_file;
#[cfg(feature = "verify")]
pub mod verify;

/// capsule_linux's own rule for slot 0 of the seal, compiled from its file.
#[path = "../../capsule_linux/src/linux/seal_slot.rs"]
pub mod seal_slot;

#[cfg(feature = "verify")]
pub use verify::{verify, Refusal, Verdict};
