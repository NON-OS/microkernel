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

//! A storage slot as a controller holds it after Configure Endpoint, and
//! its contexts read and written dword by dword.

use super::fixture::{root_place, Fixture, BULK_IN_DCI, BULK_OUT_DCI, SLOT};
use crate::dma::DmaRegion;
use crate::slots::SlotResources;

pub const CONTEXT_SIZE: u8 = 32;
const EP_STATE_STOPPED: u32 = 3;

pub fn dw(region: &DmaRegion, context: usize, dword: usize) -> u32 {
    let at = context * CONTEXT_SIZE as usize + dword * 4;
    // SAFETY: every context region holds 33 contexts of CONTEXT_SIZE bytes.
    unsafe { (region.as_mut_ptr::<u8>().add(at) as *const u32).read_volatile() }
}

pub fn set_dw(region: &DmaRegion, context: usize, dword: usize, value: u32) {
    let at = context * CONTEXT_SIZE as usize + dword * 4;
    // SAFETY: as for `dw`.
    unsafe { (region.as_mut_ptr::<u8>().add(at) as *mut u32).write_volatile(value) }
}

/// An addressed SuperSpeed slot with bulk pipes, its output context holding
/// what a controller holds after Configure Endpoint: Context Entries 5 and
/// both bulk endpoints Stopped, with a burst of 15 and a 1024-byte packet.
pub fn storage_slot(fx: &Fixture) -> SlotResources {
    let mut res = SlotResources::allocate(&fx.pool, CONTEXT_SIZE, SLOT, root_place(1, 4), true)
        .expect("slot resources");
    res.bulk = Some(fx.bulk_pipes());
    set_dw(&res.output_context, 0, 0, 5 << 27 | 4 << 20);
    for dci in [BULK_IN_DCI, BULK_OUT_DCI] {
        let ep = dci as usize;
        set_dw(&res.output_context, ep, 0, EP_STATE_STOPPED);
        set_dw(&res.output_context, ep, 1, 3 << 1 | 6 << 3 | 15 << 8 | 1024 << 16);
        set_dw(&res.output_context, ep, 2, 0xDEAD_0001);
        set_dw(&res.output_context, ep, 3, 0);
        set_dw(&res.output_context, ep, 4, 3 * 1024);
    }
    res
}
