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

//! A DMA run kept for its device's wipe: kept only when a release drains an
//! unconfined grant of a PCI function; then, in either order of the surface
//! registry's let-go and the wipe's settle, with either fate, freed exactly
//! once if and only if the end frees it, and never before the settle.

use super::super::kept_rule::keeps;
use super::kept_world::{run, Ev};

#[test]
fn only_a_released_unconfined_grant_of_a_pci_function_is_kept() {
    let mut kept = 0;
    for (pci, confined, released) in (0..8).map(|c| (c & 1 != 0, c & 2 != 0, c & 4 != 0)) {
        let want = pci && !confined && released;
        assert_eq!(keeps(pci, confined, released), want, "{pci} {confined} {released}");
        kept += want as u32;
    }
    assert_eq!(kept, 1, "an unmap by the holder, or a confined grant, is freed as before");
}

#[test]
fn freed_once_after_both_let_go_if_and_only_if_the_end_frees() {
    // Freed, kept for good, waiting for the registry, waiting for the wipe.
    let mut seen = [0u32; 4];
    for held in [false, true] {
        let orders: &[&[Ev]] = match held {
            true => &[
                &[],
                &[Ev::LetGo],
                &[Ev::Settle],
                &[Ev::LetGo, Ev::Settle],
                &[Ev::Settle, Ev::LetGo],
            ],
            false => &[&[], &[Ev::Settle]],
        };
        for (&order, frees) in orders.iter().flat_map(|o| [(o, false), (o, true)]) {
            let w = run(held, frees, order, false);
            let settled = order.contains(&Ev::Settle);
            let both = settled && (!held || order.contains(&Ev::LetGo));
            let case = format!("held {held} frees {frees} {order:?}");
            assert_eq!(w.freed, u32::from(frees && both), "{case}");
            assert!(!w.early, "{case}: freed before its wipe ended");
            assert_eq!(w.kept, u32::from(settled && !frees), "{case}");
            seen[0] += w.freed;
            seen[1] += w.kept;
            seen[2] += u32::from(frees && settled && !both);
            seen[3] += u32::from(!settled);
        }
    }
    assert_eq!(seen, [3, 4, 1, 6]);
}
