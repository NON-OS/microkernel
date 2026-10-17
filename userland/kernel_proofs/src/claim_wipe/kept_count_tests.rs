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

//! A kept run is an unconfined grant, counted as one until it is freed
//! (L80): whichever of the registry's let-go and the wipe's settle frees it
//! notes the release, once, and a run kept for good stays counted. The
//! orders are driven by the kept rule itself; which paths note a release is
//! read from the kernel.

use super::kept_rule::{on_let_go, on_settle, Act, Run};

const HELD: &str = include_str!("../../../../src/hardware/broker/dma/held_run.rs");
const SETTLE: &str = include_str!("../../../../src/hardware/broker/dma/kept_settle.rs");
const NOTE: &str = "crate::memory::iommu::note_unconfined_released(1);";

fn flat(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Whether a run freed by the let-go, and one freed by the settle, is noted.
fn noted_by() -> (bool, bool) {
    let held = flat(HELD);
    let body = &held[held.find("pub fn free_held_run").expect("free_held_run")..];
    let body = &body[..body.find("pub(super) fn free_now").expect("free_now")];
    let let_go = body.contains(&format!("if kept == Some(false) {{ {NOTE} }} free_now("));
    let settle = flat(SETTLE).contains(&format!("super::held_run::free_now(start, len); {NOTE}"));
    (let_go, settle)
}

#[test]
fn a_kept_run_is_counted_released_exactly_when_it_is_freed() {
    let (let_go_notes, settle_notes) = noted_by();
    // Freed by the let-go, freed by the settle, kept for good.
    let mut seen = [0u32; 3];
    for frees in [false, true] {
        for orders in [&[true, false][..], &[false, true][..]] {
            let mut run = Some(Run { let_go: false, settled: false });
            let (mut freed, mut noted) = (0u32, 0u32);
            for &let_go in orders {
                let Some(r) = run else { break };
                let (after, act) = if let_go { on_let_go(r) } else { on_settle(r, frees) };
                run = (act != Act::Free).then_some(after);
                if act == Act::Free {
                    freed += 1;
                    noted += u32::from(if let_go { let_go_notes } else { settle_notes });
                    seen[usize::from(!let_go)] += 1;
                }
                seen[2] += u32::from(act == Act::Keep);
            }
            assert!(freed <= 1, "{frees} {orders:?}: freed twice");
            assert_eq!(noted, freed, "{frees} {orders:?}: the count kept a freed run or lost one");
        }
    }
    assert!(seen[0] >= 1 && seen[1] >= 1 && seen[2] >= 1, "an order went untried: {seen:?}");
}
