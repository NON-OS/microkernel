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

use super::late_bars::unassigned;

#[test]
fn assigned_bars_are_usable() {
    assert!(!unassigned(&[(0x4000, 0xFE80_0000), (0x100, 0xE000)]), "memory and I/O both placed");
    assert!(!unassigned(&[]), "a function with no BARs");
}

#[test]
fn a_bar_with_a_size_and_no_base_was_never_assigned() {
    assert!(unassigned(&[(0x4000, 0xFE80_0000), (0x1000, 0)]), "the second BAR is unplaced");
    assert!(unassigned(&[(0x20, 0)]), "an I/O BAR at port 0");
    assert!(!unassigned(&[(0, 0)]), "an unimplemented BAR has no size");
}
