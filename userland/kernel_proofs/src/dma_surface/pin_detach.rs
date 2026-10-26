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

//! grant_detach.rs, snapshot.rs and orphans_free.rs as model.rs, common.rs
//! and let_go.rs read them, pinned whole: a live surface over the run that
//! another pid maps becomes an orphan holding the run with nothing to free
//! per page; an orphan goes once no pid but its owner maps it.

use super::pin_text::pinned;

const DETACH: &str =
    include_str!("../../../../src/kernel_core/surface_registry/pin/grant_detach.rs");
const SNAPSHOT: &str =
    include_str!("../../../../src/kernel_core/surface_registry/attach_map/snapshot.rs");
const FREE: &str = include_str!("../../../../src/kernel_core/surface_registry/pin/orphans_free.rs");

#[test]
fn detach_orphans_a_surface_over_the_run_another_process_maps() {
    pinned(
        DETACH,
        "detach",
        &[
            "fn detach(run: (u64, u64)) { let live: Vec<(usize, SurfaceHandle, u32)> = SLOTS",
            ".lock() .iter() .enumerate() .filter_map(|(idx, e)| { let s = e.as_ref()?;",
            "(s.grant == Some(run) && !s.frames.is_empty())",
            ".then(|| (idx, encode_handle(idx as u32, s.epoch), s.owner_pid)) }) .collect();",
            "for (idx, handle, owner) in live { let attached = has_foreign_holder(handle, owner);",
            "let mut slots = SLOTS.lock(); let Some(slot) = slots[idx].as_mut() else { continue };",
            "if encode_handle(idx as u32, slot.epoch) != handle { continue; }",
            "let frames = core::mem::take(&mut slot.frames); drop(slots);",
            "if attached { let (free, grant) = (Vec::new(), Some(run));",
            "ORPHANS.lock().push(Orphan { owner, handle, frames, free, grant, tables: 0 }); }",
            "else { release_unmapped(owner, handle); } } }",
        ],
    );
}

#[test]
fn a_holder_is_a_record_by_another_pid_and_an_orphan_goes_without_one() {
    pinned(
        SNAPSHOT,
        "has_foreign_holder",
        &[
            "fn has_foreign_holder(handle: SurfaceHandle, owner: u32) -> bool {",
            "ATTACHES.lock().iter().any(|r| r.handle == handle && r.pid != owner) }",
        ],
    );
    pinned(
        FREE,
        "free_orphans",
        &[
            "fn free_orphans() { let mut done = Vec::new(); { let mut list = ORPHANS.lock();",
            "let mut i = 0; while i < list.len() {",
            "if has_foreign_holder(list[i].handle, list[i].owner) || pending(list[i].tables) {",
            "i += 1; } else { done.push(list.swap_remove(i)); } } }",
            "for orphan in done { release_unmapped(orphan.owner, orphan.handle);",
            "for frame in orphan.free {",
            "if crate::memory::frame_alloc::deallocate_frame(frame).is_err() {",
            r#"crate::sys::serial::println(b"[SURFACE] orphan_frame_release_failed"); } } } }"#,
        ],
    );
}
