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

//! The order MkMmap backs a range in, pure so kernel_proofs holds it to its
//! rules: every frame zeroed before it is mapped, so no thread of the
//! process ever reads a frame's last owner's data through the new mapping;
//! shootdowns served before every page; and, every WINDOW_PAGES pages, a
//! window in which interrupts pending on this CPU are taken. The syscall
//! runs with interrupts masked, and a gigabyte mapped in one masked span
//! held device interrupts off this CPU for 1.5 s (cpu-6). Nothing is held
//! across a window: the busy mark is a ticket, not a lock, and the frame
//! allocator and the page tables take and drop their own locks per page. A
//! tick taken in the window never switches threads, as only a tick from
//! user mode may (interrupts/timer/tick.rs).

/// Pages between two interrupt windows: 64 KiB, the serve unit of a masked
/// loop (smp::SERVE_UNIT).
pub(crate) const WINDOW_PAGES: usize = 16;

/// The steps of backing one range.
pub(crate) trait MapOps {
    type Frame: Copy;
    fn serve_shootdowns(&mut self);
    /// Take pending interrupts, if this CPU has them masked.
    fn window(&mut self);
    fn allocate(&mut self) -> Option<Self::Frame>;
    fn zero(&mut self, frame: Self::Frame);
    /// Map page `i` of the range to `frame`.
    fn map(&mut self, i: usize, frame: Self::Frame) -> bool;
    fn free(&mut self, frame: Self::Frame);
    /// Unmap and free the first `mapped` pages of the range.
    fn rollback(&mut self, mapped: usize);
}

/// Back `pages` pages; on a failure, everything this call mapped is undone.
pub(crate) fn map_loop<O: MapOps>(ops: &mut O, pages: usize) -> bool {
    for i in 0..pages {
        if i % WINDOW_PAGES == 0 && i > 0 {
            ops.window();
        }
        ops.serve_shootdowns();
        let Some(frame) = ops.allocate() else {
            ops.rollback(i);
            return false;
        };
        ops.zero(frame);
        if !ops.map(i, frame) {
            ops.free(frame);
            ops.rollback(i);
            return false;
        }
    }
    true
}
