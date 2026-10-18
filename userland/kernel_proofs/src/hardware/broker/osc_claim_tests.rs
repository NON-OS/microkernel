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

//! F-110b, the kernel's side of _OSC: whatever the capsule publishes, the
//! record keeps no grant past the kernel's own claim, and says what it cut.
//! With the claim at 0 today, the kernel owns no PCIe control natively.

use super::super::acpi::osc_grant::*;

#[test]
fn no_grant_past_the_kernels_claim_is_recorded() {
    let s = &mut 0x9E37_79B9_7F4A_7C15u64;
    let mut roll = || {
        *s ^= *s << 13;
        *s ^= *s >> 7;
        *s ^= *s << 17;
        *s as u32 & ALL
    };
    let mut cut = 0;
    for _ in 0..20_000 {
        let (offered, extra_bits, claim) = (roll(), roll(), roll());
        let g = OscGrant { segment: 0, first: 0, last: 9, offered, granted: offered & extra_bits };
        let (kept, extra) = within_claim(g, claim);
        assert_eq!(kept.granted, g.granted & claim);
        assert_eq!(extra, g.granted & !claim);
        assert_eq!((kept.offered, kept.first, kept.last), (offered, 0, 9), "the rest kept");
        assert_eq!(kept.granted | extra, g.granted, "nothing lost from the account");
        cut += usize::from(extra != 0);
    }
    assert!(cut >= 5_000, "{cut} grants cut");
}

#[test]
fn today_the_kernel_runs_no_control_natively() {
    let g = OscGrant { segment: 0, first: 0, last: 255, offered: ALL, granted: ALL };
    let (kept, extra) = within_claim(g, KERNEL_CLAIM);
    assert_eq!((kept.granted, extra), (0, ALL));
    for c in [NATIVE_HOT_PLUG, SHPC_HOT_PLUG, PME, AER, PCIE_CAPABILITY, LTR, DPC] {
        assert!(!kept.os_owns(c), "{c:#x}");
    }
}
