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

use super::apic_plan::*;

/* ---- ICR destinations ---- */

#[test]
fn x2apic_icr_is_one_write_with_a_32_bit_destination() {
    assert_eq!(x2apic_icr(0x1_0042, 0x4041), 0x0001_0042_0000_4041);
    assert_eq!(x2apic_icr(u32::MAX - 1, 0x41) >> 32, 0xFFFF_FFFE);
}

#[test]
fn xapic_destination_is_never_truncated() {
    assert_eq!(xapic_icr_high(0), Some(0));
    assert_eq!(xapic_icr_high(0x4F), Some(0x4F00_0000));
    assert_eq!(xapic_icr_high(0xFE), Some(0xFE00_0000));
    // 0xFF is broadcast; 0x100 would truncate to the boot CPU.
    assert_eq!(xapic_icr_high(0xFF), None);
    assert_eq!(xapic_icr_high(0x100), None);
    assert!(dest_reachable(0x100, true));
    assert!(!dest_reachable(0x100, false));
}
