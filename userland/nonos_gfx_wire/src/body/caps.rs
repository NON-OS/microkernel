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

//! CAPS (0x10), 32 bytes: kind u16, outputs u16, flags u32, max width and
//! height u32, width alignment u16, reserved u16, surface budget u64 (the
//! largest frame plus the cursor slot), physical size u32 (w | h << 16 mm).

pub use super::caps_flags::*;
use crate::bytes::{put, u16_at, u32_at, u64_at};
use crate::errno::{Errno, EINVAL};

pub const CAPS_LEN: usize = 32;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Caps {
    pub kind: u16,
    pub outputs: u16,
    pub flags: u32,
    pub max_width: u32,
    pub max_height: u32,
    pub width_align: u16,
    pub surface_budget: u64,
    pub physical_mm: u32,
}

impl Caps {
    pub fn encode(&self) -> [u8; CAPS_LEN] {
        let mut b = [0u8; CAPS_LEN];
        put(&mut b, 0, &self.kind.to_le_bytes());
        put(&mut b, 2, &self.outputs.to_le_bytes());
        put(&mut b, 4, &self.flags.to_le_bytes());
        put(&mut b, 8, &self.max_width.to_le_bytes());
        put(&mut b, 12, &self.max_height.to_le_bytes());
        put(&mut b, 16, &self.width_align.to_le_bytes());
        put(&mut b, 20, &self.surface_budget.to_le_bytes());
        put(&mut b, 28, &self.physical_mm.to_le_bytes());
        b
    }

    /// Read CAPS; an adapter with no output or an alignment of 0 is not
    /// one the compositor can drive.
    pub fn decode(b: &[u8]) -> Result<Self, Errno> {
        let c = Caps {
            kind: u16_at(b, 0).ok_or(EINVAL)?,
            outputs: u16_at(b, 2).ok_or(EINVAL)?,
            flags: u32_at(b, 4).ok_or(EINVAL)?,
            max_width: u32_at(b, 8).ok_or(EINVAL)?,
            max_height: u32_at(b, 12).ok_or(EINVAL)?,
            width_align: u16_at(b, 16).ok_or(EINVAL)?,
            surface_budget: u64_at(b, 20).ok_or(EINVAL)?,
            physical_mm: u32_at(b, 28).ok_or(EINVAL)?,
        };
        if c.outputs == 0 || c.width_align == 0 {
            return Err(EINVAL);
        }
        Ok(c)
    }

    pub fn has(&self, flag: u32) -> bool {
        self.flags & flag != 0
    }
}
