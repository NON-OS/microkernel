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

//! The Setup Stage TRB every class driver's control request is sent with
//! (`setup_stage`): an 8-byte TRB Transfer Length alone in dword 2, and the
//! Transfer Type in dword 3 (xHCI 1.2 section 6.4.1.2.1). The type once sat
//! in dword 2, the length read 0x10008 or 0x18008, and qemu-xhci dropped
//! every such TRB without an event: no configuration descriptor, so no
//! keyboard, tablet or stick on xHCI.

use crate::constants::{TRB_IDT, TRB_TYPE_SETUP_STAGE, TRT_IN_DATA, TRT_NO_DATA, TRT_OUT_DATA};
use crate::trb::builders::setup_stage_generic::{setup_stage, SetupDir};

const TRT_MASK: u32 = 3 << 16;
const LENGTH_MASK: u32 = 0x1_FFFF;

#[test]
fn the_length_is_eight_and_the_transfer_type_is_in_dword_three() {
    let dirs = [
        (SetupDir::NoData, TRT_NO_DATA),
        (SetupDir::HostToDevice, TRT_OUT_DATA),
        (SetupDir::DeviceToHost, TRT_IN_DATA),
    ];
    for (dir, trt) in dirs {
        for (w_length, cycle) in [(0u16, false), (9, true), (255, false), (0xFFFF, true)] {
            let trb = setup_stage(0x80, 6, 0x0200, 0, w_length, dir, cycle);
            assert_eq!(trb.d2 & LENGTH_MASK, 8, "{dir:?}: TRB Transfer Length");
            assert_eq!(trb.d2, 8, "{dir:?}: dword 2 holds the length, interrupter 0");
            assert_eq!(trb.d3 & TRT_MASK, trt, "{dir:?}: Transfer Type");
            assert_ne!(trb.d3 & TRB_IDT, 0, "{dir:?}: immediate data");
            assert_eq!(trb.get_type(), TRB_TYPE_SETUP_STAGE);
            assert_eq!(trb.get_cycle(), cycle);
        }
    }
}

#[test]
fn the_setup_packet_is_the_request_in_wire_order() {
    let trb = setup_stage(0x21, 0x0A, 0x1234, 0x5678, 0x9ABC, SetupDir::HostToDevice, true);
    assert_eq!(trb.d0, 0x21 | 0x0A << 8 | 0x1234 << 16);
    assert_eq!(trb.d1, 0x5678 | 0x9ABC << 16);
}
