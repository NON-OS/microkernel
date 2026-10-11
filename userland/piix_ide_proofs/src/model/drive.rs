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

//! One drive in a slot: an ATA disk with its sectors and IDENTIFY DEVICE
//! data, or a packet device. The faults a test can arm.

use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Ata,
    Packet,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fault {
    None,
    /// The next DMA command ends in ERR with the engine left active.
    DmaError,
    /// Every command leaves the disk busy until a reset.
    Hang,
    /// The next DMA command ends one sector early with no error: the disk
    /// is done, the engine is left active with the table unfinished.
    Short,
    /// The next DMA command moves every sector, then reports an error.
    ErrorAtEnd,
}

pub struct Drive {
    pub kind: Kind,
    pub sectors: u64,
    pub lba48: bool,
    pub identify: [u16; 256],
    pub store: HashMap<u64, [u8; 512]>,
    pub fault: Fault,
    pub flushes: u32,
}

impl Drive {
    /// A disk as QEMU's ide-hd describes one: LBA and DMA, flush, and LBA48
    /// when asked for (QEMU sets it always; a test can clear it).
    pub fn disk(sectors: u64, lba48: bool) -> Self {
        let mut w = [0u16; 256];
        w[0] = 0x0040;
        put_text(&mut w[10..20], b"QM00001             ");
        put_text(&mut w[27..47], b"QEMU HARDDISK                           ");
        w[49] = (1 << 9) | (1 << 8);
        let lba28 = sectors.min((1 << 28) - 1);
        w[60] = lba28 as u16;
        w[61] = (lba28 >> 16) as u16;
        w[82] = 0x4000 | (1 << 5);
        w[83] = 0x4000 | (1 << 12) | if lba48 { (1 << 10) | (1 << 13) } else { 0 };
        for i in 0..4 {
            w[100 + i] = if lba48 { (sectors >> (16 * i)) as u16 } else { 0 };
        }
        Self {
            kind: Kind::Ata,
            sectors,
            lba48,
            identify: w,
            store: HashMap::new(),
            fault: Fault::None,
            flushes: 0,
        }
    }

    pub fn packet() -> Self {
        Self { kind: Kind::Packet, ..Self::disk(0, false) }
    }

    pub fn sector(&self, lba: u64) -> [u8; 512] {
        self.store.get(&lba).copied().unwrap_or([0; 512])
    }
}

/// An ATA string: two characters a word, the first in the high byte.
pub fn put_text(words: &mut [u16], text: &[u8]) {
    for (i, w) in words.iter_mut().enumerate() {
        *w = u16::from(text[2 * i]) << 8 | u16::from(text[2 * i + 1]);
    }
}
