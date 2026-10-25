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

//! What a verifier asks a container about: one listing, the policy its
//! signed release carries, a few days, and the pins it trusts.

use crate::syscall_root::SET_BYTES;
use crate::trust::Trust;

/// The fields of a LinuxAppPolicy the order reads. The policy's owner
/// decodes it (magic, version, reserved bytes); app_id is recomputed here
/// from the raw bytes, never taken from the reader.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PolicyView {
    pub program_blake3: [u8; 32],
    pub mask: u64,
    /// Zero for a vendor program.
    pub listing_id: [u8; 32],
    pub syscall_set: [u8; SET_BYTES],
}

/// `None` for bytes that are not a well-formed policy.
pub type PolicyDecode = fn(&[u8]) -> Option<PolicyView>;

/// The most days a verifier may accept at once; a wider span would let a
/// device's tag link across more than the scope promises.
pub const MAX_DAYS: u64 = 8;

pub struct Expected<'a> {
    pub listing_id: [u8; 32],
    pub policy: &'a [u8; 256],
    /// The first and last unix day accepted, inclusive.
    pub days: (u64, u64),
    pub pins: &'a Trust<'a>,
    pub read_policy: PolicyDecode,
}

impl Expected<'_> {
    pub(super) fn accepts_day(&self, window: u64) -> bool {
        accepts_day(self.days, window)
    }
}

/// `window` inside `days`, inclusive, a span of fewer than MAX_DAYS days.
pub(super) fn accepts_day((lo, hi): (u64, u64), window: u64) -> bool {
    hi >= lo && hi - lo < MAX_DAYS && (lo..=hi).contains(&window)
}
