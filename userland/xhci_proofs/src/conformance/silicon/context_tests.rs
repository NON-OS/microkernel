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

//! Input contexts as silicon reads them: 64-byte contexts at their stride,
//! and Context Entries that only rise.

use super::regions::{dw, region, set_dw};
use crate::contexts::payload::Payload;
use crate::contexts::{
    copy_slot_context, with_entries, write_configure_endpoint_input, write_evaluate_ep0_input,
    EndpointConfig,
};

#[test]
fn context_entries_only_ever_rise() {
    let dw0 = 0x0030_0000 | (5 << 27);
    assert_eq!(with_entries(dw0, 3) >> 27, 5, "a second interface keeps the first's endpoint");
    assert_eq!(with_entries(dw0, 9) >> 27, 9);
    assert_eq!(with_entries(dw0, 9) & 0x07FF_FFFF, 0x0030_0000, "the rest is unchanged");
}

#[test]
fn sixty_four_byte_contexts_are_laid_out_at_their_stride() {
    let csz = 64usize;
    let (input, output) = (region(33 * 64), region(32 * 64));
    // Output slot context: speed 3, root port 7, 1 entry; EP0 at 64 bytes.
    set_dw(&output, 0, (3 << 20) | (1 << 27));
    set_dw(&output, 4, 7 << 16);
    set_dw(&output, csz + 4, (3 << 1) | (4 << 3) | (8 << 16));
    copy_slot_context(&input, &output, csz as u8, 3);
    assert_eq!(dw(&input, csz), (3 << 20) | (3 << 27), "slot context at input index 1");
    assert_eq!(dw(&input, csz + 4), 7 << 16);

    write_evaluate_ep0_input(&input, &output, csz as u8, 64);
    assert_eq!(dw(&input, 4), 1 << 1, "only A1");
    assert_eq!(dw(&input, 2 * csz + 4) >> 16, 64, "EP0 at input index 2");
    assert_eq!(dw(&input, 2 * csz + 4) & 0xFFFF, (3 << 1) | (4 << 3), "type and CErr kept");

    let cfg = EndpointConfig {
        context_size: csz as u8,
        dci: 3,
        ring_phys: 0x1_2345_6780,
        payload: Payload { max_packet: 8, max_burst: 0, esit: 8 },
        interval: 6,
    };
    write_configure_endpoint_input(&input, &output, cfg);
    assert_eq!(dw(&input, 4), 1 | (1 << 3), "A0 and A3");
    let ep = 4 * csz;
    assert_eq!(dw(&input, ep) >> 16 & 0xFF, 6, "Interval");
    assert_eq!(dw(&input, ep + 4) >> 3 & 0x7, 7, "Interrupt IN");
    assert_eq!(dw(&input, ep + 8), 0x2345_6781, "dequeue low with DCS");
    assert_eq!(dw(&input, ep + 12), 0x1, "dequeue high: no 32-bit truncation");
    assert_eq!(dw(&input, csz) >> 27, 3, "Context Entries covers DCI 3");
}
