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

use super::window::BusWindow;

/// The Pi 4's eMMC2: RAM's first gigabyte at bus 0xc000_0000.
const EMMC2: BusWindow = BusWindow { bus_base: 0xc000_0000, cpu_base: 0, size: 0x4000_0000 };

#[test]
fn a_span_is_moved_only_when_all_of_it_is_inside() {
    assert_eq!(EMMC2.bus_address(0, 0x1000), Some(0xc000_0000));
    assert_eq!(EMMC2.bus_address(0x3fff_f000, 0x1000), Some(0xffff_f000));
    assert_eq!(EMMC2.bus_address(0x3fff_f000, 0x2000), None);
    assert_eq!(EMMC2.bus_address(0x4000_0000, 0), Some(0x1_0000_0000));
    let high = BusWindow { bus_base: 0, cpu_base: 0x4000_0000, size: 0x1000 };
    assert_eq!(high.bus_address(0x3fff_f000, 0x1000), None);
    let top = BusWindow { bus_base: u64::MAX - 0xfff, cpu_base: 0, size: 0x2000 };
    assert_eq!(top.bus_address(0x1000, 0x1000), None);
    assert_eq!(
        BusWindow { bus_base: 0, cpu_base: 0, size: u64::MAX }.bus_address(u64::MAX - 1, 1),
        Some(u64::MAX - 1)
    );
}

#[test]
fn only_a_window_ending_by_4_gib_takes_low_frames() {
    assert!(EMMC2.ends_below_4g());
    assert!(BusWindow { bus_base: 0, cpu_base: 0, size: 1 << 32 }.ends_below_4g());
    assert!(!BusWindow { bus_base: 0, cpu_base: 1, size: 1 << 32 }.ends_below_4g());
    assert!(!BusWindow { bus_base: 0, cpu_base: 0, size: u64::MAX }.ends_below_4g());
}
