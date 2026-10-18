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

//! A capability list that loops still ends, and the USB Legacy Support
//! capability is found where real firmware leaves it.

use super::super::model::{controller, LEGACY, MAPPED, OS_OWNED, XECP_WORDS};
use super::caps::{lay_out_caps, supported_protocol};
use crate::controller::legacy_handoff;
use crate::regs::cap::PortProtocols;

#[test]
fn the_legacy_capability_is_found_behind_the_protocol_capabilities() {
    let bar = controller(XECP_WORDS);
    let legacy = LEGACY + 0x40;
    lay_out_caps(&bar, &[(LEGACY, supported_protocol(2), 1 | (2 << 8)), (legacy, 1, 0)]);
    bar.present32(legacy + 4, 0);
    legacy_handoff(bar.base(), MAPPED);
    assert_ne!(bar.wrote32(legacy) & OS_OWNED, 0, "the OS asked for the controller");
}

#[test]
fn a_legacy_capability_past_the_mapping_is_not_touched() {
    let bar = controller(XECP_WORDS);
    bar.present32(LEGACY, 1);
    bar.present32(LEGACY + 4, 0xFFFF_FFFF);
    // Mapped short of the capability's second dword.
    legacy_handoff(bar.base(), LEGACY as u64 + 4);
    assert_eq!(bar.wrote32(LEGACY), 1, "nothing was written");
    assert_eq!(bar.wrote32(LEGACY + 4), 0xFFFF_FFFF);
}

#[test]
fn a_capability_list_that_loops_ends() {
    let hcc = (0x100u32 << 16) | 1;
    // Next pointer zero would end it; a pointer back to itself cannot, as
    // the offset only grows, so loop through a run of pointers of one.
    let ports = PortProtocols::parse(hcc, u64::MAX, |_| supported_protocol(3) | (1 << 8));
    // Every read gives the same capability: port 2 (dw2 bits 7:0), one port.
    assert_eq!(ports.major(2), 3);
    assert_eq!(ports.major(0), 0, "port 0 is never a port");
}
