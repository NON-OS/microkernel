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

//! The `MkBootSlots` record: the bootloader's slot, then the kernel's, at fixed
//! offsets, all integers little-endian.
//!
//! | offset | bytes | field |
//! |---|---|---|
//! | 0 | 4 | version, 1 |
//! | 4 | 4 | zero |
//! | 8 | 380 | the bootloader's slot |
//! | 388 | 380 | the kernel's slot |
//!
//! A slot: kind (3 bootloader, 0 kernel), depth, six zero bytes, the epoch as
//! eight bytes, then the measurement, the context digest and the root, 32
//! bytes each, then the v3 path, then two zero bytes.

use super::slot::{Slot, PATH_LEN};
use nonos_boot_measure::gate::DEPTH;

pub const RECORD_VERSION: u32 = 1;
pub const SLOT_LEN: usize = 380;
pub const RECORD_LEN: usize = 8 + 2 * SLOT_LEN;

/* Every field of a slot fits, with the two zero bytes that close it. */
const _: () = assert!(112 + PATH_LEN + 2 == SLOT_LEN);

/// `bootloader` then `kernel`, at their offsets.
pub fn encode(bootloader: &Slot, kernel: &Slot) -> [u8; RECORD_LEN] {
    let mut out = [0u8; RECORD_LEN];
    out[..4].copy_from_slice(&RECORD_VERSION.to_le_bytes());
    put(&mut out[8..8 + SLOT_LEN], bootloader);
    put(&mut out[8 + SLOT_LEN..], kernel);
    out
}

fn put(out: &mut [u8], s: &Slot) {
    out[0] = s.kind;
    out[1] = DEPTH as u8;
    out[8..16].copy_from_slice(&s.epoch.to_le_bytes());
    out[16..48].copy_from_slice(&s.measurement);
    out[48..80].copy_from_slice(&s.digest);
    out[80..112].copy_from_slice(&s.root);
    out[112..112 + PATH_LEN].copy_from_slice(&s.path);
}
