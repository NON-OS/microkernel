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

//! The CONTROLLER_INFO payload.

use super::super::sdhci::{Caps, Snapshot};
use super::CONTROLLER_INFO_LEN;

pub fn controller_info(caps: &Caps, snap: &Snapshot) -> [u8; CONTROLLER_INFO_LEN] {
    let mut o = [0u8; CONTROLLER_INFO_LEN];
    let ghc = snap.host_control as u32 | (snap.power as u32) << 8 | (snap.clock as u32) << 16;
    o[0..4].copy_from_slice(&caps.caps.to_le_bytes());
    o[4..8].copy_from_slice(&ghc.to_le_bytes());
    o[8..12].copy_from_slice(&1u32.to_le_bytes());
    o[12..16].copy_from_slice(&(caps.version as u32).to_le_bytes());
    o[16..20].copy_from_slice(&caps.caps1.to_le_bytes());
    o[20] = 1;
    o
}
