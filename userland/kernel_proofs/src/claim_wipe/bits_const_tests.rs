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

//! The register bits a wipe reads and writes (L78) are the ones the plan
//! names, every one recalled from the PCIe base specification and unverified,
//! and those sharing a register never overlap. A slip here would send the
//! reset, the marker or the wait to a bit that means something else; pinning
//! them makes any edit of one a deliberate one.

use super::bits_rule::*;

#[test]
fn each_bit_is_the_one_the_plan_names() {
    // All (recalled, unverified).
    assert_eq!((FLR_CAP, INITIATE_FLR, NO_SNOOP_EN), (1 << 28, 1 << 15, 1 << 11));
    assert_eq!(TRANSACTIONS_PENDING, 1 << 5);
    assert_eq!((AF_TP_CAP, AF_FLR_CAP, AF_INITIATE_FLR, AF_TP), (1, 2, 1, 1));
    assert_eq!((PM_STATE, D3HOT, NO_SOFT_RESET, PME_STATUS), (0x3, 3, 1 << 3, 1 << 15));
    assert_eq!((IO, MEM, BME, INTX_DISABLE), (1, 1 << 1, 1 << 2, 1 << 10));
}

#[test]
fn bits_sharing_a_register_never_overlap() {
    let command = [IO, MEM, BME, INTX_DISABLE];
    let pmcsr = [PM_STATE, NO_SOFT_RESET, PME_STATUS];
    let af = [AF_TP_CAP, AF_FLR_CAP].map(u16::from);
    for set in [&command[..], &pmcsr[..], &[INITIATE_FLR, NO_SNOOP_EN][..], &af[..]] {
        for (i, a) in set.iter().enumerate() {
            assert_ne!(*a, 0);
            for b in &set[i + 1..] {
                assert_eq!(a & b, 0, "{a:#x} and {b:#x} overlap");
            }
        }
    }
    assert_eq!(D3HOT & !PM_STATE, 0, "D3hot lies inside the PowerState field");
}
