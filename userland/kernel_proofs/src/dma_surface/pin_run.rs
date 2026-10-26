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

//! grant_run.rs as model.rs and let_go.rs read it, pinned whole: the
//! release takes only the run, detaches the live surfaces over it and holds
//! it while an orphan over it is listed; a let-go frees each run no orphan
//! holds, once. teardown_pin_tests pins the teardown's call to `hold`.

use super::pin_text::pinned;

const RUN: &str = include_str!("../../../../src/kernel_core/surface_registry/pin/grant_run.rs");

#[test]
fn hold_takes_only_the_run_and_keeps_it_while_an_orphan_holds_it() {
    pinned(
        RUN,
        "hold",
        &[
            "fn hold(start: u64, length: u64) -> bool { let run = (start, length);",
            "let _gate = lock_responsive(&GATE); super::grant_detach::detach(run);",
            "let held = ORPHANS.lock().iter().any(|o| o.grant == Some(run));",
            "if held { RUNS.lock().push(run); } held }",
        ],
    );
}

#[test]
fn free_unheld_gives_back_each_run_no_orphan_holds_once() {
    pinned(
        RUN,
        "free_unheld",
        &[
            "fn free_unheld() { let mut free = Vec::new(); { let _gate = lock_responsive(&GATE);",
            "let held: Vec<(u64, u64)> = ORPHANS.lock().iter().filter_map(|o| o.grant).collect();",
            "RUNS.lock().retain(|run| { let keep = held.contains(run);",
            "if !keep { free.push(*run); } keep }); }",
            "for (start, length) in free {",
            "crate::hardware::broker::dma_free_held_run(start, length); } }",
        ],
    );
}
