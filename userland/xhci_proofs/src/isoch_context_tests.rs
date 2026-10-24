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

//! The Configure Endpoint input an isoch OUT endpoint is added with, read
//! back by field as the controller reads it (xHCI 1.2 6.2.3, as recalled),
//! the Isoch TRB's bits, and the batches an isoch OUT request is refused.

use crate::controller::{batch_fits, isoch_endpoint, write_isoch_input, IsochEndpoint};
use crate::dma::{DmaPool, DmaRegion};
use crate::trb::builders::isoch::isoch;

const CONTEXT_SIZE: u8 = 32;

fn dw(r: &DmaRegion, context: usize, dword: usize) -> u32 {
    let at = context * CONTEXT_SIZE as usize + dword * 4;
    // SAFETY: the region holds 33 contexts of CONTEXT_SIZE bytes.
    unsafe { core::ptr::read_volatile(r.as_mut_ptr::<u8>().add(at) as *const u32) }
}

#[test]
fn the_endpoint_context_is_an_isoch_out_one_that_never_retries() {
    let pool = DmaPool::new(1, 1);
    let (input, output) = (pool.alloc(33 * 32).unwrap(), pool.alloc(32 * 32).unwrap());
    output.zero();
    let ep =
        IsochEndpoint { ring_phys: 0x1_0000_2000, ..isoch_endpoint(4, true, 4, 4, (3, 1), 1024) };
    write_isoch_input(&input, &output, CONTEXT_SIZE, ep);
    assert_eq!(dw(&input, 0, 1), 1 | (1 << 4), "the slot and DCI 4 added");
    assert_eq!((dw(&input, 1, 0) >> 27) & 0x1F, 4, "Context Entries reach DCI 4");
    let (dw0, dw1) = (dw(&input, 5, 0), dw(&input, 5, 1));
    assert_eq!(((dw0 >> 8) & 0x3, (dw0 >> 16) & 0xFF, dw0 >> 24), (1, 3, 0), "Mult, Interval");
    assert_eq!((dw1 >> 1) & 0x3, 0, "CErr 0: no retries");
    assert_eq!((dw1 >> 3) & 0x7, 1, "Isoch OUT");
    assert_eq!(((dw1 >> 8) & 0xFF, dw1 >> 16), (3, 1024), "Max Burst, Max Packet Size");
    assert_eq!((dw(&input, 5, 2), dw(&input, 5, 3)), (0x2001, 1), "the ring, DCS set");
    assert_eq!(dw(&input, 5, 4), 8192 | (8192 << 16), "average TRB and ESIT payload");
}

#[test]
fn a_batch_the_endpoint_cannot_carry_is_refused() {
    let esit = 192;
    assert!(batch_fits(&[192, 96], &[0; 288], esit));
    assert!(!batch_fits(&[], &[], esit), "no packet");
    assert!(!batch_fits(&[192; 32], &[0; 192 * 32], 4096), "past 31 packets");
    assert!(!batch_fits(&[0, 192], &[0; 192], esit), "an empty packet");
    assert!(!batch_fits(&[193], &[0; 193], esit), "past one interval's payload");
    assert!(!batch_fits(&[192], &[0; 191], esit), "lengths and bytes disagree");
    assert!(!batch_fits(&[4096, 1], &[0; 4097], 8192), "past the half");
}

#[test]
fn an_isoch_trb_starts_as_soon_as_possible_and_carries_its_bursts() {
    let t = isoch(0x1_2345_6000, 192, 1, 2, true, true);
    assert_eq!((t.get_pointer(), t.d2 & 0x1_FFFF), (0x1_2345_6000, 192));
    assert_eq!((t.d3 >> 10) & 0x3F, 5, "Isoch TRB type");
    assert!(t.d3 & (1 << 31) != 0, "SIA");
    assert_eq!((t.d3 >> 7) & 0x3, 1, "TBC");
    assert_eq!((t.d3 >> 16) & 0xF, 2, "TLBPC");
    assert!(t.d3 & (1 << 5) != 0 && t.d3 & 1 == 1, "IOC and cycle");
    let quiet = isoch(0, 192, 0, 0, false, false);
    assert!(quiet.d3 & (1 << 5) == 0 && quiet.d3 & 1 == 0);
}
