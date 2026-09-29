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

use super::numa::NumaMemoryRegion;
use super::range::PortRange;
use super::srat_memory::SratMemoryAffinity;

const TOP_PAGE: u64 = 0xFFFF_FFFF_FFFF_F000;

#[test]
fn port_ranges_reaching_0xffff_hold_it() {
    assert!(PortRange::new(0xFFFF, 1).contains(0xFFFF));
    assert!(PortRange::new(0xFFF0, 16).contains(0xFFFF));
    assert!(PortRange::new(0xFFF0, 16).overlaps(&PortRange::new(0xFFFF, 1)));
    assert!(PortRange::new(0xFFFF, 1).overlaps(&PortRange::new(0xFFF0, 16)));
    assert!(!PortRange::new(0x60, 1).overlaps(&PortRange::new(0x61, 1)));
    assert!(!PortRange::new(0x60, 0).contains(0x60));
}

#[test]
fn srat_entry_reaching_the_top_holds_its_last_byte() {
    let entry = SratMemoryAffinity {
        entry_type: 1,
        length: 40,
        proximity_domain: 0,
        reserved1: 0,
        base_address: TOP_PAGE,
        length_bytes: 0x1000,
        reserved2: 0,
        flags: SratMemoryAffinity::ENABLED,
        reserved3: 0,
    };
    assert!(entry.contains_address(u64::MAX));
    assert!(entry.contains_address(TOP_PAGE));
    assert!(!entry.contains_address(TOP_PAGE - 1));
}

#[test]
fn numa_region_reaching_the_top_holds_its_last_byte() {
    let region = NumaMemoryRegion {
        base: TOP_PAGE,
        length: 0x1000,
        proximity_domain: 3,
        hot_pluggable: false,
        non_volatile: false,
    };
    assert!(region.contains(u64::MAX));
    assert!(!region.contains(TOP_PAGE - 1));
}
