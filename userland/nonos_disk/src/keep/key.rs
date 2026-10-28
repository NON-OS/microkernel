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

//! The key header's way, and whether the volume it keys outlives the
//! kernel it was made under. The installed system's kernel writes the
//! header (`KEY_LBA`, the sector after the plan); the installer only reads
//! it, and an update leaves it as it found it.
//!
//! ```text
//!   0..8      "NONOSDK1"
//!   8         the way: 1 a TPM key bound to the kernel, 2 a passphrase,
//!             3 a TPM key wrapped under the release-approved chain
//!   128..136  "NONOSHO1" while a handover is pending, under way 3 alone
//! ```

use nonos_disk_map::KEY_MAGIC;

use super::why::NotKept;

const TPM_BOUND: u8 = 1;
const PASSPHRASE: u8 = 2;
const TPM_WRAPPED: u8 = 3;

/// The handover the installed system leaves when the person prepares it
/// for an update; the new kernel takes it on the first boot that uses it.
const HANDOVER_AT: usize = 128;
const HANDOVER_MAGIC: [u8; 8] = *b"NONOSHO1";

/// What the header lets an update do with the volume it keys.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Key {
    /// A new release opens it: a passphrase, or a wrapped key handed over.
    Opens,
    /// A wrapped key not handed over: the installed system must prepare it.
    Unprepared,
}

/// `s` is the key header's sector.
pub fn key_state(s: &[u8]) -> Result<Key, NotKept> {
    if s.len() < HANDOVER_AT + HANDOVER_MAGIC.len() || s[..8] != KEY_MAGIC {
        return Err(NotKept::NoKeyHeader);
    }
    let handed_over = s[HANDOVER_AT..HANDOVER_AT + HANDOVER_MAGIC.len()] == HANDOVER_MAGIC;
    match s[8] {
        PASSPHRASE => Ok(Key::Opens),
        TPM_WRAPPED if handed_over => Ok(Key::Opens),
        TPM_WRAPPED => Ok(Key::Unprepared),
        /*
         * Bound to the kernel that made it, so no other kernel derives it:
         * a handover's magic beside it opens nothing.
         */
        TPM_BOUND => Err(NotKept::MadeBefore),
        way => Err(NotKept::UnknownWay(way)),
    }
}
