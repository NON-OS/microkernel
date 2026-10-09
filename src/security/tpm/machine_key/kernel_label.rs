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

//! Keys the kernel derives for itself.
//!
//! `CryptoMachineKey` hands any capsule holding Crypto the key for a label it
//! names. A key the kernel keeps for itself therefore needs a label no syscall
//! can ask for: every kernel label starts with a zero byte, and a label from a
//! syscall that starts with one is refused.

extern crate alloc;

use alloc::vec::Vec;

use super::consts::DIGEST_LEN;
use super::derive::derive;
use super::error::KeyError;

const KERNEL_PREFIX: u8 = 0;

/// The machine key for a name only the kernel uses.
pub fn derive_for_kernel(name: &[u8]) -> Result<[u8; DIGEST_LEN], KeyError> {
    let mut label = Vec::with_capacity(1 + name.len());
    label.push(KERNEL_PREFIX);
    label.extend_from_slice(name);
    derive(&label)
}

/// True when a caller may ask for the key under `label`.
pub fn is_user_label(label: &[u8]) -> bool {
    label.first() != Some(&KERNEL_PREFIX)
}
