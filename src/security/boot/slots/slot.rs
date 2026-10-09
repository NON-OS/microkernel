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

//! One boot slot as the device proof's witness takes it: the context digest
//! of an enrolled image, the path that places it, and the root that path folds
//! to. Built from the measurement here, never read from the trailer, exactly
//! as the gate that admitted the image built it.

use nonos_attest_path::{boot_context, context_digest, parse_v4, root_of, Kind, MAX_PROOF_V4};
use nonos_boot_measure::gate::{BOOT_EPOCH, DEPTH};

/// The v3 path inside a v4 trailer at the boot trees' depth: magic, depth,
/// eight siblings, one byte of directions.
pub const PATH_LEN: usize = 9 + DEPTH * 32 + DEPTH.div_ceil(8);

/// The circuit's kind for a kernel slot and for a bootloader slot.
pub const KIND_KERNEL: u8 = 0;
pub const KIND_BOOTLOADER: u8 = 3;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Slot {
    pub kind: u8,
    pub epoch: u64,
    pub measurement: [u8; 32],
    pub digest: [u8; 32],
    pub root: [u8; 32],
    pub path: [u8; PATH_LEN],
}

/// The slot of the image measured as `measurement` and carrying `trailer`.
/// `None` when the trailer is not a v4 trailer of that kind, or its path does
/// not have the boot trees' shape.
pub fn slot(kind: Kind, measurement: &[u8; 32], trailer: &[u8]) -> Option<Slot> {
    let code = match kind {
        Kind::Kernel => KIND_KERNEL,
        Kind::Bootloader => KIND_BOOTLOADER,
        _ => return None,
    };
    let v = parse_v4(trailer, kind, MAX_PROOF_V4)?;
    let ctx = boot_context(measurement, BOOT_EPOCH);
    Some(Slot {
        kind: code,
        epoch: BOOT_EPOCH,
        measurement: *measurement,
        digest: context_digest(&ctx)?,
        root: root_of(DEPTH, kind, &ctx, v.path)?,
        path: v.path.try_into().ok()?,
    })
}
