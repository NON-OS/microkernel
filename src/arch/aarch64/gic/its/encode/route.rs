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

//! The commands that name a redistributor: mapping the collection every
//! LPI is delivered through, and waiting for earlier commands to land there.

use super::cmd::{Cmd, MAPC, SYNC, VALID};

const ADDR_51_16: u64 = 0x000f_ffff_ffff_0000;

/// The redistributor a command names, as bits 51:16 of its third word: its
/// physical address when `GITS_TYPER.PTA` says the ITS takes addresses, else
/// its processor number, which Linux shifts into the same field.
pub const fn target(pta: bool, rd_phys: u64, processor: u16) -> u64 {
    if pta {
        rd_phys & ADDR_51_16
    } else {
        (processor as u64) << 16
    }
}

pub const fn mapc(collection: u16, target: u64) -> Cmd {
    [MAPC, 0, VALID | target | collection as u64, 0]
}

/// Complete every earlier command's effect at `target` before this one
/// is consumed.
pub const fn sync(target: u64) -> Cmd {
    [SYNC, 0, target, 0]
}
