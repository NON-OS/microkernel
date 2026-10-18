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

//! The input context a command names, read the way the controller reads
//! it: through the bus address in the command TRB.

use nonos_libc::dma_host;

use super::hc::Hc;
use crate::trb::{read_volatile_at, Trb};

/// 32-byte contexts: the Input Control Context, the Slot Context, EP0.
const CONTEXT: u64 = 32;

/// A command the controller ran, and the input context it named: the
/// Input Control Context's drop and add flags, then the Slot Context.
#[derive(Clone, Copy, Debug)]
pub struct Ran {
    pub trb: Trb,
    pub input: [u32; 6],
}

impl Hc {
    /// Dword `dw` of context `index` of the input context `trb` names.
    pub(super) fn input_dw(&self, trb: &Trb, index: u64, dw: u64) -> u32 {
        let at = (trb.get_pointer() & !0xF) + index * CONTEXT + dw * 4;
        // SAFETY: `dma_host` names live host memory behind the bus address.
        dma_host(at).map_or(0, |host| unsafe { (host as *const u32).read_volatile() })
    }

    /// The Input Control Context's two flag dwords and the Slot Context.
    pub(super) fn input_of(&self, trb: &Trb) -> [u32; 6] {
        let mut out = [0u32; 6];
        if trb.get_pointer() == 0 {
            return out;
        }
        for (i, v) in out.iter_mut().enumerate() {
            *v = if i < 2 {
                self.input_dw(trb, 0, i as u64)
            } else {
                self.input_dw(trb, 1, i as u64 - 2)
            };
        }
        out
    }
}

/// The TRB at `host`, which `dma_host` named behind a ring.
pub(super) fn trb_at(host: u64) -> Trb {
    // SAFETY: `dma_host` names live host memory, a whole TRB of the ring.
    unsafe { read_volatile_at(host) }
}
