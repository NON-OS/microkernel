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

/* ---- LINT0/LINT1 from MADT NMI entries (types 4 and 0xA) ---- */

#[test]
fn lint_defaults_mask_lint0_and_give_the_bsp_nmi_on_lint1() {
    assert_eq!(lint_lvts(&[], Some(0), true), (LVT_MASKED, LVT_DM_NMI));
    assert_eq!(lint_lvts(&[], Some(1), false), (LVT_MASKED, LVT_MASKED));
}

#[test]
fn lint_follows_madt_entries_for_this_uid_or_all() {
    let all = LintNmi { processor_uid: u32::MAX, lint: 1, flags: 0b0101 };
    assert_eq!(lint_lvts(&[all], Some(3), false), (LVT_MASKED, LVT_DM_NMI));

    // Per-processor entries, as HP and Lenovo firmware lists them.
    let per = [
        LintNmi { processor_uid: 1, lint: 1, flags: 0b1111 },
        LintNmi { processor_uid: 2, lint: 1, flags: 0b0101 },
    ];
    assert_eq!(lint_lvts(&per, Some(1), true), (LVT_MASKED, LVT_DM_NMI | LVT_ACTIVE_LOW));
    assert_eq!(lint_lvts(&per, Some(2), false), (LVT_MASKED, LVT_DM_NMI));
    // A CPU no entry names keeps the defaults.
    assert_eq!(lint_lvts(&per, Some(9), false), (LVT_MASKED, LVT_MASKED));
    // A nonsense pin is ignored.
    let bad = LintNmi { processor_uid: u32::MAX, lint: 7, flags: 0 };
    assert_eq!(lint_lvts(&[bad], None, false), (LVT_MASKED, LVT_MASKED));
}
