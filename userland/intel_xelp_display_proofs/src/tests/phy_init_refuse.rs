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
//! PHY initialisation refused: B before A, and a PHY with no procmon column.

use super::phy_init::{fresh, A, B};
use nonos_intel_xelp_display::phy::init::init;
use nonos_intel_xelp_display::regs::clka::Combo;

#[test]
fn b_waits_for_a_and_sets_no_irefgen() {
    let mut m = fresh();
    assert!(init(&mut m, Combo::B).is_err());
    assert!(m.writes.is_empty());
    init(&mut m, Combo::A).unwrap();
    init(&mut m, Combo::B).unwrap();
    assert_eq!(m.get(B + 0x120) & (1 << 24), 0);
    assert_eq!(m.get(B + 0x100) & (1 << 31), 1 << 31);
}

#[test]
fn unknown_process_refused_untouched() {
    let mut m = fresh();
    m.set(A + 0x10C, (2 << 26) | (1 << 24)); // dot-4: no column
    assert!(init(&mut m, Combo::A).is_err());
    assert!(m.writes.is_empty());
}
