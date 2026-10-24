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
//! EHCI's legacy support, at the offset its id says, and nowhere else.
//! The EHCI hand-off rule over a whole config space: legacy support found
//! by its id only outside the standard capability list, so the Power
//! Management capability (also id 01h) and its PMCSR stay closed.

use super::quirk_bits::{legsup_before, writable, Ident};

/// A config space with the capability list present and `caps` as
/// (offset, id, next) entries, then `bytes` written over it.
pub(super) fn space(caps: &[(u8, u8, u8)], bytes: &[(u8, u8)]) -> [u8; 256] {
    let mut c = [0u8; 256];
    c[0x06] = 0x10;
    c[0x34] = caps.first().map_or(0, |e| e.0);
    for &(at, id, next) in caps {
        (c[at as usize], c[at as usize + 1]) = (id, next);
    }
    for &(at, v) in bytes {
        c[at as usize] = v;
    }
    c
}

/// What the rule allows at `offset` on an EHCI function with config `c`.
pub(super) fn allows(c: &[u8; 256], offset: u32) -> Option<u16> {
    let read = |o: u16| c.get(o as usize).copied().unwrap_or(0xFF);
    let (vendor, class, subclass, prog_if) = (0x8086, 0x0C, 0x03, 0x20);
    let before = legsup_before(&read, offset);
    writable(&Ident { vendor, class, subclass, prog_if, pcie_cap: None, before }, offset)
}

#[test]
fn a_power_management_capability_where_an_eecp_could_be_stays_closed() {
    let c = space(&[(0x68, 0x01, 0x00)], &[]);
    assert_eq!(allows(&c, 0x6C), None, "PMCSR: bits 0-1 are the power state");
    for offset in [0x6A, 0x6E] {
        assert_eq!(allows(&c, offset), None, "{offset:#x}");
    }
}

#[test]
fn the_real_legacy_support_bits_beside_a_listed_pm_stay_open() {
    // ICH-like: PM at 0x50, the debug port at 0x58, EECP at 0x68.
    let c = space(&[(0x50, 0x01, 0x58), (0x58, 0x0A, 0x00)], &[(0x68, 0x01)]);
    assert_eq!(allows(&c, 0x6A), Some(0x0100), "OS owned");
    assert_eq!(allows(&c, 0x6C), Some(0xE03F), "SMI enables");
    assert_eq!(allows(&c, 0x6E), Some(0xE000), "SMI status");
    for offset in [0x52, 0x54, 0x56, 0x5A, 0x68] {
        assert_eq!(allows(&c, offset), None, "{offset:#x}");
    }
}
