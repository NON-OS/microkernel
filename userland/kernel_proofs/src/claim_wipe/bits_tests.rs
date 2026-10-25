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

//! Device Control and PMCSR as a wipe writes them (L78), over every value
//! read. Device Control put back never asks a second reset nor turns no-snoop
//! on for the next holder; a power-state write sets the state asked, never
//! writes PME_Status as 1 (that clears a wake the next driver should see),
//! and keeps every other bit as read.

use super::bits_rule::{devctl_after, pmcsr_to};

/// Initiate FLR and Enable No Snoop (recalled, unverified).
const DEVCTL_OURS: u16 = 1 << 15 | 1 << 11;
/// PME_Status and the PowerState field (recalled, unverified).
const PMCSR_OURS: u16 = 1 << 15 | 0x3;

#[test]
fn device_control_put_back_clears_only_the_reset_and_no_snoop_bits() {
    let mut cleared = [0u32; 2];
    for before in 0..=u16::MAX {
        let after = devctl_after(before);
        assert_eq!(after & DEVCTL_OURS, 0, "{before:#06x}: a reset or no-snoop survives");
        assert_eq!(after & !DEVCTL_OURS, before & !DEVCTL_OURS, "{before:#06x}: a bit lost");
        cleared[0] += u32::from(before & 1 << 15 != 0);
        cleared[1] += u32::from(before & 1 << 11 != 0);
    }
    assert_eq!(cleared, [32_768; 2], "half of all values carry each bit in");
}

#[test]
fn a_power_state_write_sets_the_state_exactly_and_never_pme_status() {
    let mut pme_read = 0u32;
    for read in 0..=u16::MAX {
        for state in 0..4u16 {
            let w = pmcsr_to(read, state);
            assert_eq!(w & 0x3, state, "{read:#06x} to D{state}");
            assert_eq!(w & 1 << 15, 0, "{read:#06x} to D{state}: PME_Status written as 1");
            assert_eq!(w & !PMCSR_OURS, read & !PMCSR_OURS, "{read:#06x} to D{state}");
        }
        pme_read += u32::from(read & 1 << 15 != 0);
    }
    assert_eq!(pme_read, 32_768, "half of all reads carry a pending wake");
}

/// The build keeps only the PowerState field of the state asked (the plan
/// wrote it whole), so no stray bit asked reaches PME_Status or the read's.
#[test]
fn only_the_state_field_of_what_is_asked_is_written() {
    let (mut x, mut stray) = (0x9E37_79B9_7F4A_7C15u64, 0u32);
    for state in 0..=u16::MAX {
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        let read = x as u16;
        let w = pmcsr_to(read, state);
        assert_eq!(w & 0x3, state & 0x3, "{read:#06x} asked {state:#06x}");
        assert_eq!(w & !PMCSR_OURS, read & !PMCSR_OURS, "{read:#06x} asked {state:#06x}");
        assert_eq!(w & 1 << 15, 0, "{read:#06x} asked {state:#06x}");
        stray += u32::from(state & !0x3 & !read != 0);
    }
    assert!(stray > 64_000, "a stray bit the read lacked was asked {stray} times");
}
