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

//! The disk plan: one plain sector saying where the data volume lies and
//! where the files to import wait.
//!
//! The host or an installer writes it, so nothing in it is trusted: every
//! range is checked against the disk and against every other range before
//! it is used. It carries no hash; what an import must hash to comes from
//! the signed capsule that asks for it, never from the disk.
//!
//!   bytes  0..8   magic "NONOSDP1"
//!          8..16  volume base LBA       16..24  volume sectors
//!         24..32  import count, at most MAX_IMPORTS
//!         32..    per import: its LBA, then its length in bytes
//!
//! A plan whose volume base and sectors are both 0 is a live stick's: it
//! keeps no volume on the disk, so the session's stays in RAM and nothing of
//! this machine's is written to the stick, and it only names files to import,
//! as the release stick names the model tier it carries.

/// The plan's own sector, 120 MiB in: past the store, whose window holds the
/// staged packages and the wallpapers read only when opened.
pub const PLAN_LBA: u64 = 245_760;
/// Nothing the plan names may start below 128 MiB.
pub const DATA_FLOOR: u64 = 262_144;
/// A volume needs its 256-sector header ring, a root and one block more.
pub(super) const MIN_VOLUME: u64 = 258;
pub(super) const MAGIC: [u8; 8] = *b"NONOSDP1";
/// As many 16-byte entries as fit after the 32-byte head.
pub const MAX_IMPORTS: usize = 30;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Plan {
    pub volume_base: u64,
    pub volume_sectors: u64,
    /// Where each file to import starts, and its length in bytes.
    pub imports: [(u64, u64); MAX_IMPORTS],
    pub count: usize,
}

impl Plan {
    /// A live stick's plan: no volume on the disk, only imports.
    pub fn is_live(&self) -> bool {
        self.volume_base == 0 && self.volume_sectors == 0
    }

    pub fn imports(&self) -> &[(u64, u64)] {
        &self.imports[..self.count]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlanError {
    /// The sector does not start with the plan's magic.
    NoPlan,
    BelowFloor,
    VolumeTooSmall,
    /// A range runs past the end of the disk, or its end overflows.
    PastEnd,
    /// Two ranges, the volume or an import, share sectors.
    Overlap,
    /// More imports than the sector has room for, or an empty one.
    BadImport,
}
