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

//! The hub fields Configure Hub writes into a Slot Context (xHCI 1.2
//! section 6.2.2): Hub in dword 0 bit 26, MTT in bit 25, Number of Ports in
//! dword 1 bits 31:24, TT Think Time in dword 2 bits 17:16. TT fields are a
//! high-speed hub's only.

use crate::contexts::slot::hub_dwords;
use crate::contexts::HubInfo;

const HUB: u32 = 1 << 26;
const MTT: u32 = 1 << 25;

/// A Slot Context with every other field set, so a write that strays shows.
const OTHERS: [u32; 3] = [0xF_FFFF | 0xF << 20 | 0x1F << 27, 0xFF_FFFF, 0xFFC0_FFFF];

#[test]
fn a_high_speed_hub_gets_hub_mtt_ports_and_think_time() {
    let hub = HubInfo { ports: 15, think_time: 3, mtt: true };
    let [dw0, dw1, dw2] = hub_dwords(OTHERS, hub, true);
    assert_eq!(dw0, OTHERS[0] | HUB | MTT);
    assert_eq!(dw1, OTHERS[1] | 15 << 24);
    assert_eq!(dw2, OTHERS[2] | 3 << 16);
}

#[test]
fn any_other_hub_gets_hub_and_ports_and_no_tt_fields() {
    let hub = HubInfo { ports: 4, think_time: 3, mtt: true };
    let [dw0, dw1, dw2] = hub_dwords([OTHERS[0] | MTT, OTHERS[1], OTHERS[2] | 3 << 16], hub, false);
    assert_eq!(dw0, OTHERS[0] | HUB, "MTT cleared");
    assert_eq!(dw1, OTHERS[1] | 4 << 24);
    assert_eq!(dw2, OTHERS[2], "think time cleared");
}

#[test]
fn values_past_their_field_never_reach_a_neighbour() {
    let hub = HubInfo { ports: 255, think_time: 255, mtt: false };
    let [_, dw1, dw2] = hub_dwords([0, 0, 0], hub, true);
    assert_eq!(dw1, 255 << 24);
    assert_eq!(dw2, 3 << 16);
}
