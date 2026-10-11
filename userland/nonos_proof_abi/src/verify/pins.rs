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

//! What a verifier brings: the device trust roots it accepts, the listing
//! and days it asks about, the isolation statement it pins, and a reader for
//! the app's LinuxAppPolicy. Nothing here comes from the container.

use nox_verify::Statement;

use crate::syscall_root::SET_BYTES;

/// One accepted release: its bootloader and kernel trees, and the device
/// registry at the depth it was published.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct DeviceRoots {
    pub boot_root: [u64; 4],
    pub kernel_root: [u64; 4],
    pub device_root: [u64; 4],
    pub device_depth: usize,
}

/// The two fields of a LinuxAppPolicy the order reads. The policy's owner
/// decodes it (magic, version, reserved bytes); app_id is recomputed here
/// from the raw bytes, never taken from the reader.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PolicyView {
    pub program_blake3: [u8; 32],
    pub syscall_set: [u8; SET_BYTES],
}

/// `None` for bytes that are not a well-formed policy.
pub type PolicyDecode = fn(&[u8]) -> Option<PolicyView>;

/// The most days a verifier may accept at once; a wider window would let
/// one device's tag link across more than the scope promises.
pub const MAX_DAYS: u64 = 8;

pub struct Pins<'a> {
    pub roots: &'a [DeviceRoots],
    pub listing_id: [u8; 32],
    /// The first and last unix day accepted, inclusive.
    pub days: (u64, u64),
    pub isolation: &'a Statement,
    pub policy: PolicyDecode,
}
