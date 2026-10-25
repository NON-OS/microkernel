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

//! The kernel's randomness callers, mounted where the kernel has them, so
//! their `crate::` and `super::` paths resolve as they do there. Their real
//! crypto is mounted too (ChaCha20-Poly1305, SHA-256, HMAC and PBKDF2,
//! BLAKE3, Argon2id, the constant-time helpers). Only the seeded RNG, the
//! clock, the log, the devices and the keyring stand in (`stand_in`).

#![cfg(test)]
#![allow(dead_code)]

extern crate alloc;

pub mod stand_in;
pub mod engine_mount;
pub use stand_in::{arch, boot, hardware, log, memory, smp, sys, time};

include!("mount_crypto.rs");
include!("mount_fs.rs");
include!("mount_security.rs");

mod tests;
