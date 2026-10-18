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

//! The test's side of an I/O queue: the queue built by the driver's own
//! allocate, and completions posted into its ring the way the controller
//! writes them.

use super::geometry::NamespaceGeometry;
use super::queue::IoQueue;
use crate::admin::Completion;

/// An I/O queue on host memory: namespace 1, 512-byte LBAs, the queue's
/// whole data buffer per command.
pub(super) fn io_queue() -> IoQueue {
    let g =
        NamespaceGeometry { nsid: 1, capacity_sectors: 1 << 20, lba_size: 512, max_sectors: 64 };
    match IoQueue::allocate(7, 1, 0, 1, &g) {
        Ok(io) => io,
        Err(e) => panic!("host memory refused: {e:?}"),
    }
}

/// Post `c` at `slot` of the queue's completion ring, as the controller does.
pub(super) fn post(io: &IoQueue, slot: u16, c: Completion) {
    assert!(slot < 8, "slot {slot} is past the ring");
    let at = io.cq.user_va() + slot as u64 * 16;
    // SAFETY: the ring is the queue's own completion region, 8 entries of
    // 16 bytes inside its 4 KiB.
    unsafe { core::ptr::write_volatile(at as *mut Completion, c) };
}
