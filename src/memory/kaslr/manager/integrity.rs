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

//! Whether what KASLR published still holds.

use super::super::constants::{INTEGRITY_CHECK_BUFFER_SIZE, INTEGRITY_CHECK_LABEL};
use super::derive::derive_subkey;
use super::init::get_slide;
use super::validate::validate;
use crate::memory::layout;

/// True when the state validates, the two records of the slide agree, and the
/// key derivation still answers.
pub fn verify_slide_integrity() -> bool {
    if validate().is_err() {
        return false;
    }
    /*
     * The manager's slide and the layout record's slide are two copies of one
     * value, written by different code. They agree only if `apply_kaslr_slide`
     * ran for the slide the manager chose, so a divergence is a layout moved
     * by a different amount than the kernel believes, or not moved at all
     * while the kernel believes it was. The line this replaces compared
     * `KERNEL_BASE + slide` against `KERNEL_BASE + slide` and could not fail.
     */
    if get_slide() != layout::get_slide() {
        return false;
    }
    let mut buffer = [0u8; INTEGRITY_CHECK_BUFFER_SIZE];
    if derive_subkey(INTEGRITY_CHECK_LABEL, &mut buffer).is_err() {
        return false;
    }
    /*
     * A dead derivation leaves the buffer as it was handed over. Anything
     * else is a hash output and is allowed to look like one.
     *
     * The previous form required all 64 bytes to be non-zero, which a real
     * hash satisfies about 78% of the time, so it reported corruption on
     * roughly one boot in five. It also rejected an XOR fold of 0x00 or 0xFF,
     * two values out of 256 that a sound derivation produces as readily as
     * any other.
     */
    buffer.iter().any(|&b| b != 0)
}
