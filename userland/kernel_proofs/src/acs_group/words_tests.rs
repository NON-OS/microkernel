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

//! K11b: the ACS words `redirect` reads, by layout, from a register model:
//! offset 4 the capability (control above it in the standard layout),
//! offset 8 the control dword on Sunrise Point.

use std::cell::RefCell;

use super::acs_layout::Layout;
use super::acs_rule::{FUNCTION, PORT};
use super::acs_words::words;

/// The registers past an ACS header; the capability dword is read-only
/// where `ro_cap`, as on Sunrise Point.
pub(super) struct Regs {
    pub r: RefCell<[u32; 3]>,
    pub ro_cap: bool,
}

pub(super) fn read(regs: &Regs, layout: Layout, needed: u16) -> Option<(u16, u16)> {
    let rd = |o: u16| regs.r.borrow()[usize::from(o / 4)];
    let wr = |o: u16, v: u32| {
        if !(o == 4 && regs.ro_cap) {
            regs.r.borrow_mut()[usize::from(o / 4)] = v;
        }
    };
    words(layout, needed, true, rd, wr)
}

#[test]
fn the_standard_layout_is_turned_on_in_its_control_word() {
    let regs = Regs { r: RefCell::new([0, u32::from(PORT), 0]), ro_cap: false };
    assert_eq!(read(&regs, Layout::Standard, PORT), Some((PORT, PORT)));
    assert_eq!(regs.r.borrow()[1], u32::from(PORT) | u32::from(PORT) << 16);
}

#[test]
fn sunrise_point_is_turned_on_at_its_control_dword() {
    let spt = || Regs { r: RefCell::new([0, u32::from(PORT), 0]), ro_cap: true };
    let regs = spt();
    assert_eq!(read(&regs, Layout::SptDword, PORT), Some((PORT, PORT)));
    assert_eq!(regs.r.borrow()[2], u32::from(PORT));
    // Read the standard way, its control is the capability's high half.
    assert_eq!(read(&spt(), Layout::Standard, PORT), Some((PORT, 0)));
}

#[test]
fn an_intel_rciep_redirects_without_a_register_read() {
    let never = |_: u16| -> u32 { panic!("an RCiEP is not read") };
    let got = words(Layout::Rciep, FUNCTION, false, never, |_, _| panic!("nor written"));
    assert_eq!(got, Some((FUNCTION, FUNCTION)));
    assert_eq!(words(Layout::Standard, FUNCTION, false, never, |_, _| {}), None, "no ACS");
}
