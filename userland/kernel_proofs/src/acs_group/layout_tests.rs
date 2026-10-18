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

//! K11b (2) and (3): which ACS layout a function has, by who made it.

use super::acs_layout::{layout, Layout};

const INTEL: u16 = 0x8086;
const ROOT: Option<u8> = Some(4);
const RC_END: Option<u8> = Some(9);
const ENDPOINT: Option<u8> = Some(0);

#[test]
fn the_linux_spt_root_port_ids_have_dword_acs() {
    let spt = [0xA110, 0xA11F, 0xA167, 0xA16A, 0xA290, 0xA29F, 0xA2E7, 0xA2EE, 0x9D10, 0x9D1B];
    for d in spt {
        assert_eq!(layout(INTEL, d, ROOT, 0), Layout::SptDword, "{d:#x}");
    }
    let near = [0xA10F, 0xA120, 0xA166, 0xA16B, 0xA28F, 0xA2A0, 0xA2E6, 0xA2EF, 0x9D0F, 0x9D1C];
    for d in near.into_iter().chain([0x9DB8, 0x8C10]) {
        assert_eq!(layout(INTEL, d, ROOT, 0), Layout::Standard, "{d:#x}");
    }
    assert_eq!(layout(0x1022, 0xA110, ROOT, 0), Layout::Standard, "another vendor");
    assert_eq!(layout(INTEL, 0xA110, ENDPOINT, 0), Layout::Standard, "not a root port");
}

#[test]
fn intel_root_complex_endpoints_on_bus_0_redirect() {
    assert_eq!(layout(INTEL, 0x9DF0, RC_END, 0), Layout::Rciep, "CNVi");
    assert_eq!(layout(INTEL, 0x9DED, RC_END, 0), Layout::Rciep, "xHCI");
    assert_eq!(layout(INTEL, 0x9DF0, RC_END, 1), Layout::Standard, "not on bus 0");
    assert_eq!(layout(0x1AF4, 0x1042, RC_END, 0), Layout::Standard, "QEMU's virtio");
    assert_eq!(layout(INTEL, 0x2922, None, 0), Layout::Standard, "conventional PCI");
}
