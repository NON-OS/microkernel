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

//! The PCI function a PCI_Config region's path reaches.

use crate::exec::PciPath;
use crate::platform::function_of;

#[test]
fn a_region_behind_a_bridge_reaches_the_secondary_bus() {
    let path = PciPath { segment: 0, bus: 0, adrs: vec![0x001C_0000, 0x0000_0000] };
    let mut reads = Vec::new();
    let mut sec = |f| {
        reads.push(f);
        Some(2)
    };
    assert_eq!(function_of(&path, &mut sec), Some((0, 2, 0, 0)));
    assert_eq!(reads, [(0, 0, 0x1C, 0)]);
    let flat = PciPath { segment: 0, bus: 0, adrs: vec![0x001F_0003] };
    assert_eq!(function_of(&flat, &mut |_| None), Some((0, 0, 0x1F, 3)));
    let bad = PciPath { segment: 0, bus: 0, adrs: vec![0x0020_0000] };
    assert_eq!(function_of(&bad, &mut |_| None), None);
    assert_eq!(function_of(&PciPath { segment: 0, bus: 0, adrs: vec![] }, &mut |_| None), None);
}
