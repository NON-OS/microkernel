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

//! Each driver's request magic and opcodes, copied from its `protocol`
//! module; the proofs and the drivers' own tests hold the two together.

use super::table::Driver;

/// The longest capacity body any dialect answers with: the block wire's
/// GEOMETRY, 20 bytes. The others answer 8, a block count alone.
pub const CAPACITY_BODY_MAX: usize = 20;

/// Opcodes in the order capacity, read, write, flush. Every request goes to
/// unit 0, the word the header leaves zero; each driver serves one disk.
pub struct Ops {
    pub capacity: u16,
    pub read: u16,
    pub write: u16,
    pub flush: u16,
}

impl Driver {
    pub fn magic(self) -> u32 {
        match self {
            Driver::VirtioBlk => 0x4E42_4C4B,
            Driver::Nvme => 0x4E4E_564D,
            Driver::Ahci => 0x4E41_4843,
            Driver::Rtsx => 0x4E53_4452,
            // One arm each: scripts/check_blk_wire.py reads a row per driver.
            Driver::Wire => 0x4E42_5731,
            Driver::Ufs => 0x4E42_5731,
            Driver::Ide => 0x4E42_5731,
            Driver::Scsi => 0x4E42_5731,
        }
    }

    pub fn ops(self) -> Ops {
        match self {
            Driver::VirtioBlk => Ops { capacity: 2, read: 3, write: 4, flush: 5 },
            Driver::Nvme => Ops { capacity: 6, read: 7, write: 8, flush: 9 },
            Driver::Ahci => Ops { capacity: 4, read: 5, write: 6, flush: 7 },
            Driver::Rtsx => Ops { capacity: 2, read: 3, write: 4, flush: 5 },
            // GEOMETRY opens with the block count, as a capacity reply does.
            // Both serve the one block wire; an arm each, so the table reads
            // row by row (scripts/check_blk_wire.py).
            Driver::Wire => Ops { capacity: 2, read: 3, write: 4, flush: 5 },
            Driver::Ufs => Ops { capacity: 2, read: 3, write: 4, flush: 5 },
            Driver::Ide => Ops { capacity: 2, read: 3, write: 4, flush: 5 },
            Driver::Scsi => Ops { capacity: 2, read: 3, write: 4, flush: 5 },
        }
    }
}
