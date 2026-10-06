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

//! The `MkBootSlots` record, read. Its layout is documented where the kernel
//! writes it, `src/security/boot/slots/record.rs`, and a host test holds both.

pub const BOOT_SLOTS_LEN: usize = 768;
/// The v3 path: `NZKPATH1`, the depth, eight siblings, one direction byte.
pub const PATH_LEN: usize = 266;
const SLOT_LEN: usize = 380;
const VERSION: u32 = 1;
const DEPTH: u8 = 8;
const PATH_MAGIC: &[u8; 8] = b"NZKPATH1";

/// One slot; `kind` is the circuit's: 3 the bootloader, 0 the kernel.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct BootSlot {
    pub kind: u8,
    pub epoch: u64,
    pub measurement: [u8; 32],
    /// The context digest, the circuit's `Slot::digest`, and the root its
    /// path folds to, the statement's boot or kernel root.
    pub digest: [u8; 32],
    pub root: [u8; 32],
    pub path: [u8; PATH_LEN],
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct BootSlots {
    pub bootloader: BootSlot,
    pub kernel: BootSlot,
}

/// Strict: any other length, version, slot order, depth or path magic, or a
/// nonzero reserved byte, is refused.
pub fn parse_boot_slots(r: &[u8]) -> Option<BootSlots> {
    if r.len() != BOOT_SLOTS_LEN || r.get(..4)? != VERSION.to_le_bytes() || r.get(4..8)? != [0; 4] {
        return None;
    }
    Some(BootSlots {
        bootloader: slot(r.get(8..8 + SLOT_LEN)?, 3)?,
        kernel: slot(r.get(8 + SLOT_LEN..)?, 0)?,
    })
}

fn slot(s: &[u8], kind: u8) -> Option<BootSlot> {
    if s.first()? != &kind || s.get(1)? != &DEPTH || s.get(2..8)? != [0; 6] {
        return None;
    }
    let path: [u8; PATH_LEN] = s.get(112..112 + PATH_LEN)?.try_into().ok()?;
    if path.get(..8)? != PATH_MAGIC || path[8] != DEPTH || s.get(112 + PATH_LEN..)? != [0; 2] {
        return None;
    }
    Some(BootSlot {
        kind,
        epoch: u64::from_le_bytes(s.get(8..16)?.try_into().ok()?),
        measurement: s.get(16..48)?.try_into().ok()?,
        digest: s.get(48..80)?.try_into().ok()?,
        root: s.get(80..112)?.try_into().ok()?,
        path,
    })
}
