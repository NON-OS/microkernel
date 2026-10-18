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

//! The legacy device answering what it is told, when a test has it
//! answer: on a notify it takes the request the driver made available,
//! writes its status byte, and uses its head with the length it reports,
//! through the ring at the page number it was given and the addresses in
//! the chain, as a device does. A legacy device may report the status
//! byte alone (bhyve's does: one).

use nonos_libc::{dma_read, dma_write};

use super::legacy_dev::LegacyBlk;
use crate::queue::{avail_offset, used_offset};

const PAGE: u64 = 4096;
const DESC_BYTES: u64 = 16;
const F_NEXT: u16 = 1;
/// A request's chain is three descriptors at the most.
const CHAIN: usize = 3;

impl LegacyBlk {
    pub(super) fn notify(&mut self) {
        self.notified += 1;
        if let Some((len, status)) = self.answers {
            self.answer(len, status);
        }
    }

    fn answer(&self, len: u32, status: u8) {
        let ring = u64::from(self.pfn) * PAGE;
        let avail = ring + avail_offset(self.qnum) as u64;
        let used = ring + used_offset(self.qnum) as u64;
        let taken = word(used + 2);
        if taken == word(avail + 2) {
            return;
        }
        let pos = u64::from(taken % self.qnum);
        let head = word(avail + 4 + 2 * pos);
        let mut desc = ring + DESC_BYTES * u64::from(head);
        for _ in 1..CHAIN {
            if word(desc + 12) & F_NEXT == 0 {
                break;
            }
            desc = ring + DESC_BYTES * u64::from(word(desc + 14));
        }
        let mut addr = [0u8; 8];
        assert!(dma_read(desc, &mut addr), "a descriptor outside every grant");
        assert!(dma_write(u64::from_le_bytes(addr), &[status]), "a status outside every grant");
        let elem = used + 4 + 8 * pos;
        assert!(dma_write(elem, &u32::from(head).to_le_bytes()));
        assert!(dma_write(elem + 4, &len.to_le_bytes()));
        assert!(dma_write(used + 2, &taken.wrapping_add(1).to_le_bytes()));
    }
}

fn word(phys: u64) -> u16 {
    let mut b = [0u8; 2];
    assert!(dma_read(phys, &mut b), "the ring is outside every grant");
    u16::from_le_bytes(b)
}
