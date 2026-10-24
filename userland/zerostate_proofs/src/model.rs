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

//! The one line of the kernel's plan read the host cannot mount, stated
//! apart: `plan_keeps_volume` in src/fs/blockfs_volume/plan_read.rs reads
//! the sector through the block layer, then maps the parse as below. The
//! chain tests hold this mapping and the mounted rule together.

use crate::plan::parse_plan;
use crate::plan_types::PlanError;

/// `plan_keeps_volume` for a sector already read: a device fault is the
/// `None` case too, which `read` stands for.
pub fn keeps_volume(read: Option<&[u8; 512]>, capacity: u64) -> Option<bool> {
    match read.map(|s| parse_plan(s, capacity)) {
        Some(Ok(plan)) => Some(!plan.is_live()),
        Some(Err(PlanError::NoPlan)) => Some(false),
        _ => None,
    }
}

/// A plan sector: magic, volume base and sectors, imports.
pub fn plan(base: u64, sectors: u64, imports: &[(u64, u64)]) -> [u8; 512] {
    let mut s = [0u8; 512];
    s[..8].copy_from_slice(b"NONOSDP1");
    let head = [base, sectors, imports.len() as u64];
    let words = head.iter().chain(imports.iter().flat_map(|(a, b)| [a, b]));
    for (i, v) in words.enumerate() {
        s[8 + i * 8..16 + i * 8].copy_from_slice(&v.to_le_bytes());
    }
    s
}
