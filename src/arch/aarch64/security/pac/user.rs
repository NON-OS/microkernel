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

//! A process's own IA key, installed whenever its CPU enters EL0 for it and
//! replaced by the kernel's on every exception from EL0. With one key for
//! both, a process could have PACIA sign any pointer as the kernel would,
//! and a forged return address would pass the kernel's check.
//!
//! The key is a keyed hash of the address space, so every thread of a
//! process shares it and no two live processes do: a pointer one process
//! signed does not authenticate in another.

use super::slot::set_user;
use super::state::{active, user_secret};
use crate::crypto::hash::blake3_keyed_hash;

/// Before `aarch64_enter_user` or `aarch64_resume_user` for the process
/// with thread group `tgid` and translation root `root`.
pub fn enter_process(tgid: u32, root: u64) {
    if !active() {
        return;
    }
    let mut input = [0u8; 12];
    input[..4].copy_from_slice(&tgid.to_le_bytes());
    input[4..].copy_from_slice(&root.to_le_bytes());
    let hash = blake3_keyed_hash(&user_secret(), &input);
    let word = |at: usize| u64::from_le_bytes(hash[at..at + 8].try_into().unwrap_or([0; 8]));
    set_user([word(0), word(8)]);
}
