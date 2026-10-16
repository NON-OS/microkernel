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

//! MkAcpiRestoreFunction's answer before anything is written (L64): id 0
//! or an unknown function first, then an unclaimed one, then one not in
//! D0; only a known, claimed function in D0 (or with no power management
//! capability, always in D0) is restored.

use super::acpi::marf_rule::{check, Marf};

#[test]
fn only_a_known_claimed_function_in_d0_is_restored() {
    let pms = [None, Some(0u16), Some(1), Some(2), Some(3), Some(0x8), Some(0xFFFC)];
    for id in [0u64, 1, 0xDEAD] {
        for known in [false, true] {
            for claimed in [false, true] {
                for pm in pms {
                    let want = if id == 0 || !known {
                        Err(Marf::Unknown)
                    } else if !claimed {
                        Err(Marf::NotClaimed)
                    } else if pm.is_some_and(|p| p & 3 != 0) {
                        Err(Marf::NotInD0)
                    } else {
                        Ok(())
                    };
                    assert_eq!(
                        check(id, known, claimed, pm),
                        want,
                        "{id} {known} {claimed} {pm:?}"
                    );
                }
            }
        }
    }
    assert_eq!(check(0, true, true, None), Err(Marf::Unknown), "id 0 names nothing");
    assert_eq!(check(7, true, true, Some(0x8)), Ok(()), "No_Soft_Reset is not a state");
}
