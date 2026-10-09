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

//! The input context of an Evaluate Context command that changes EP0's max
//! packet size (xHCI 1.2 sections 4.6.7 and 6.2.3): only A1 is set, and the
//! EP0 context is the device's current one with the new size.

use super::slot_copy::{read_dw, write_dw};
use crate::dma::DmaRegion;

const ADD_EP0: u32 = 1 << 1;
const INPUT_EP0: usize = 2;
const OUTPUT_EP0: usize = 1;
const EP_DWORDS: usize = 5;
const MPS_SHIFT: u32 = 16;

pub fn write_evaluate_ep0_input(
    input: &DmaRegion,
    output: &DmaRegion,
    context_size: u8,
    max_packet: u16,
) {
    input.zero();
    write_dw(input, context_size, 0, 1, ADD_EP0);
    for dw in 0..EP_DWORDS {
        let mut v = read_dw(output, context_size, OUTPUT_EP0, dw);
        if dw == 1 {
            v = (v & 0xFFFF) | ((max_packet as u32) << MPS_SHIFT);
        }
        write_dw(input, context_size, INPUT_EP0, dw, v);
    }
}
