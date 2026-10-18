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
//! A bulk endpoint's streams (xHCI 1.2, 4.12, as recalled, unverified): a
//! linear Primary Stream Array of 2^n 16-byte Stream Contexts, each but the
//! reserved first naming a ring of its own (SCT 1, primary, with its cycle
//! bit). UAS moves each command on the stream its tag names.

use alloc::vec::Vec;

use crate::dma::{DmaPool, DmaRegion};
use crate::error::{XhciError, XhciResult};
use crate::rings::transfer::TransferRing;

const CONTEXT_BYTES: usize = 16;
const SCT_PRIMARY_TR: u64 = 1 << 1;
pub const MAX_LOG2: u8 = 4; // at most 15 streams, a ring of a page each

pub struct StreamPipe {
    pub array: DmaRegion,
    /// Stream `s`'s ring at `s - 1`.
    pub rings: Vec<TransferRing>,
    pub log2: u8,
}

impl StreamPipe {
    /// 2^`log2` entries, `log2` 1 to MAX_LOG2.
    pub fn new(pool: &DmaPool, log2: u8) -> XhciResult<Self> {
        if !(1..=MAX_LOG2).contains(&log2) {
            return Err(XhciError::ControllerUnsupported);
        }
        let entries = 1usize << log2;
        let array = pool.alloc((entries * CONTEXT_BYTES) as u64)?;
        array.zero();
        let mut rings = Vec::new();
        for sid in 1..entries {
            let ring = TransferRing::new(pool)?;
            let at = ring.phys() | SCT_PRIMARY_TR | ring.cycle() as u64;
            // SAFETY: entry `sid` lies inside the array just allocated.
            unsafe {
                let p = array.as_mut_ptr::<u8>().add(sid * CONTEXT_BYTES) as *mut u32;
                p.write_volatile(at as u32);
                p.add(1).write_volatile((at >> 32) as u32);
            }
            rings.push(ring);
        }
        Ok(Self { array, rings, log2 })
    }

    pub fn ring_mut(&mut self, stream: u16) -> Option<&mut TransferRing> {
        self.rings.get_mut((stream as usize).checked_sub(1)?)
    }

    /// The endpoint context's MaxPStreams: the array holds 2^(it + 1).
    pub fn max_pstreams(&self) -> u8 {
        self.log2 - 1
    }

    /// Stream ids the class driver may use, 1 to this.
    pub fn usable(&self) -> u16 {
        (1u16 << self.log2) - 1
    }
}
