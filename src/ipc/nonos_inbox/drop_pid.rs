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

//! The inboxes a dying process leaves, emptied and zeroed as they go.

use core::sync::atomic::Ordering;

use super::registry::{GLOBAL_STATS, REGISTRY};

/// Drop the canonical per-process inboxes `proc.{pid}` and `stdin.{pid}`
/// for a dying capsule, zeroing what is still queued in them: a child's
/// output and its input can be someone's private text. Called from
/// `process::exit::teardown`. Returns the count dropped from `proc.{pid}`.
/// Reply inboxes (`endpoint.<u64>`) are kernel-owned and intentionally left
/// alone so a respawn reuses them; stale replies are filtered by the
/// transport's generation re-check.
pub fn unregister_for_pid(pid: u32) -> Option<usize> {
    let _ = unregister_stdin_for_pid(pid);
    remove_zeroed(&alloc::format!("proc.{}", pid))
}

/// Drop `stdin.{pid}` alone, zeroing what is queued in it. Nothing reads a
/// dead process's input, even while its output is kept for its parent.
pub fn unregister_stdin_for_pid(pid: u32) -> Option<usize> {
    remove_zeroed(&alloc::format!("stdin.{}", pid))
}

fn remove_zeroed(module: &str) -> Option<usize> {
    let inbox = REGISTRY.write().map.remove(module)?;
    GLOBAL_STATS.total_inboxes_removed.fetch_add(1, Ordering::Relaxed);
    let mut dropped = 0;
    while let Some(mut msg) = inbox.dequeue() {
        crate::crypto::secure_zero(&mut msg.data);
        dropped += 1;
    }
    Some(dropped)
}
