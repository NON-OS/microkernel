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

//! exit_owned.rs and orphans.rs as exit_model.rs and reclaim.rs read them,
//! pinned whole: an exit orphans each surface another pid maps, listing no
//! page of a run, nor of any grant of its process, to free per page; and
//! every let-go frees the orphans under GATE, then the runs after it.

use super::pin_text::pinned;

const EXIT: &str = include_str!("../../../../src/kernel_core/surface_registry/pin/exit_owned.rs");
const ORPHANS: &str = include_str!("../../../../src/kernel_core/surface_registry/pin/orphans.rs");

#[test]
fn an_exit_strips_its_process_grants_from_the_orphans_it_makes() {
    pinned(
        EXIT,
        "orphan_owned",
        &[
            "fn orphan_owned(group: u32, tables: u64) -> Vec<PhysAddr> {",
            "let dma: Vec<(u32, u64, u64)> = crate::hardware::broker::dma::grant_ranges()",
            ".into_iter() .filter(|&(holder, _, _)| same_process(holder, group)) .collect();",
            "let mut kept = Vec::new(); for (idx, handle) in owned_by(group) {",
            "let mut slots = SLOTS.lock(); let Some(slot) = slots[idx].as_mut() else { continue };",
            "if encode_handle(idx as u32, slot.epoch) != handle || slot.owner_pid != group {",
            "continue; } let frames = core::mem::take(&mut slot.frames); let grant = slot.grant;",
            "drop(slots); if frames.is_empty() || !has_foreign_holder(handle, group) { continue; }",
            "kept.extend_from_slice(&frames);",
            "let mut free = if grant.is_some() { Vec::new() } else { frames.clone() };",
            "for &(_, start, len) in &dma { strip(&mut free, |f| f.as_u64(), start, len); }",
            "ORPHANS.lock().push(Orphan { owner: group, handle, frames, free, grant, tables }); }",
            "kept }",
        ],
    );
}

#[test]
fn every_let_go_frees_the_orphans_then_the_runs() {
    pinned(
        ORPHANS,
        "reclaim",
        &[
            "fn reclaim() { reclaim_in(|| lock_responsive(&GATE), |_| free_orphans(),",
            "super::grant_run::free_unheld); }",
        ],
    );
}
