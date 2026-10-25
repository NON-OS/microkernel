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

//! CPUID's id is recorded when the local APIC agrees or is not up yet, and
//! refused with both ids when they differ, in either APIC mode.

use super::boot_id_rule::reconcile;

#[test]
fn without_a_local_apic_cpuid_is_taken() {
    for id in [0, 1, 4, 0xFF, 0x100, 0xFFFF_FFFE] {
        assert_eq!(reconcile(id, None, false), Ok(id));
        assert_eq!(reconcile(id, None, true), Ok(id));
    }
}

// The case that halted: a boot CPU that is not APIC 0. It is recorded as
// what it is, not as 0.
#[test]
fn a_boot_cpu_that_is_not_apic_zero_keeps_its_id() {
    assert_eq!(reconcile(2, Some(2), false), Ok(2));
    assert_eq!(reconcile(0x40, Some(0x40), false), Ok(0x40));
    assert_eq!(reconcile(0x1_0000, Some(0x1_0000), true), Ok(0x1_0000));
}

#[test]
fn ids_that_differ_are_refused_with_both() {
    assert_eq!(reconcile(2, Some(0), false), Err((2, 0)));
    assert_eq!(reconcile(0, Some(2), true), Err((0, 2)));
}

// An xAPIC id register holds eight bits; a CPUID id past 255 cannot be that
// register's CPU even when its low byte matches.
#[test]
fn an_id_past_255_is_refused_in_xapic_mode() {
    assert_eq!(reconcile(0x101, Some(0x01), false), Err((0x101, 0x01)));
    assert_eq!(reconcile(0x101, Some(0x101), true), Ok(0x101));
}
